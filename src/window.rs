use core::{
    future::poll_fn,
    marker::PhantomData,
    ptr::NonNull,
    task::{Poll, Waker},
};

use crate::{
    bindings::{self, GColor, WindowHandlers},
    executor::{wake_from_ptr, waker_as_ptr},
};

pub struct WindowHandle<'active> {
    inner: NonNull<bindings::Window>,
    _phantom: PhantomData<&'active ()>,
}

impl<'active> WindowHandle<'active> {
    pub fn set_background_colour(&mut self, colour: GColor) {
        unsafe {
            bindings::window_set_background_color(self.inner.as_ptr(), colour);
        }
    }

    fn root_layer(&mut self) -> () {
        unsafe {
            bindings::window_get_root_layer(self.inner.as_ptr());
        }
    }
}

unsafe extern "C" fn window_handler_wake(window: *mut bindings::Window) {
    let ptr = unsafe { bindings::window_get_user_data(window) };
    crate::debug!("About to wake waker: {:?}", ptr);
    if let Some(waker) = NonNull::new(ptr) {
        wake_from_ptr(waker);
    }

    unsafe {
        crate::executor::poll_executor();
    }
}

unsafe extern "C" fn window_handler_noop(_window: *mut bindings::Window) {
    unsafe {
        crate::executor::poll_executor();
    }
}

/// Create a window, your passed async function will be called once with the
/// handle to the window handle, and will then be polled while the window is
/// active. When the window unloads the future will be dropped.
pub async fn with_window(f: impl for<'active> AsyncFnOnce(WindowHandle<'active>)) -> Option<()> {
    let p = unsafe { bindings::window_create() };
    let p = NonNull::new(p)?;
    let fut = f(WindowHandle {
        inner: p,
        _phantom: PhantomData,
    });

    let mut has_started: bool = false;

    crate::debug!("With window start");

    // wait for window to start
    poll_fn(|cx| unsafe {
        if !has_started {
            bindings::window_set_user_data(p.as_ptr(), waker_as_ptr(cx.waker()).as_ptr());

            bindings::window_set_window_handlers(
                p.as_ptr(),
                WindowHandlers {
                    load: Some(window_handler_wake),
                    appear: Some(window_handler_noop),
                    disappear: Some(window_handler_noop),
                    unload: Some(window_handler_noop),
                },
            );

            bindings::window_stack_push(p.as_ptr(), true);

            has_started = true;

            Poll::Pending
        } else {
            Poll::Ready(())
        }
    })
    .await;

    crate::debug!("With window created");

    let mut has_started: bool = false;
    let wait_for_stop = poll_fn(|cx| unsafe {
        if !has_started {
            bindings::window_set_user_data(p.as_ptr(), waker_as_ptr(cx.waker()).as_ptr());

            bindings::window_set_window_handlers(
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
            if bindings::window_stack_get_top_window() == p.as_ptr() {
                bindings::window_stack_pop(true);
            }
        }
    })
    .await;

    unsafe {
        bindings::window_destroy(p.as_ptr());
    }

    crate::debug!("With window destroy");

    Some(())
}
