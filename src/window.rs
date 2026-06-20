use core::{
    future::poll_fn,
    marker::PhantomData,
    ptr::NonNull,
    task::{Poll, Waker},
};

use crate::bindings::WindowHandlers;

pub struct WindowHandle<'active> {
    inner: NonNull<crate::bindings::Window>,
    _phantom: PhantomData<&'active ()>,
}

unsafe extern "C" fn window_handler_wake(window: *mut crate::bindings::Window) {
    let waker = unsafe { crate::bindings::window_get_user_data(window).cast::<Waker>() };
    if let Some(waker) = unsafe { waker.as_ref() } {
        waker.wake_by_ref();
    }
}

unsafe extern "C" fn window_handler_noop(_window: *mut crate::bindings::Window) {}

pub async fn with_window(f: impl for<'active> AsyncFnOnce(WindowHandle<'active>)) -> Option<()> {
    let p = unsafe { crate::bindings::window_create() };
    let p = NonNull::new(p)?;
    let fut = f(WindowHandle {
        inner: p,
        _phantom: PhantomData,
    });

    let mut has_started: bool = false;

    // wait for window to start
    poll_fn(|cx| unsafe {
        if !has_started {
            crate::bindings::window_set_user_data(p.as_ptr(), cx.waker() as *const _ as *mut _);

            crate::bindings::window_set_window_handlers(
                p.as_ptr(),
                WindowHandlers {
                    load: Some(window_handler_wake),
                    appear: Some(window_handler_noop),
                    disappear: Some(window_handler_noop),
                    unload: Some(window_handler_noop),
                },
            );

            crate::bindings::window_stack_push(p.as_ptr(), true);

            has_started = true;

            Poll::Pending
        } else {
            Poll::Ready(())
        }
    })
    .await;

    let mut has_started: bool = false;
    let wait_for_stop = poll_fn(|cx| unsafe {
        if !has_started {
            crate::bindings::window_set_user_data(p.as_ptr(), cx.waker() as *const _ as *mut _);

            crate::bindings::window_set_window_handlers(
                p.as_ptr(),
                WindowHandlers {
                    load: Some(window_handler_noop),
                    appear: Some(window_handler_noop),
                    disappear: Some(window_handler_noop),
                    unload: Some(window_handler_wake),
                },
            );

            has_started = true;

            Poll::Pending
        } else {
            Poll::Ready(())
        }
    });

    embassy_futures::select::select(wait_for_stop, async {
        fut.await;
        unsafe {
            if crate::bindings::window_stack_get_top_window() == p.as_ptr() {
                crate::bindings::window_stack_pop(true);
            }
        }
    })
    .await;

    Some(())
}
