#![no_std]
#![feature(integer_casts)]
#![feature(integer_widen_truncate)]
#![feature(impl_trait_in_assoc_type)]
#![feature(async_fn_traits)]

pub mod executor;
pub mod log_impl;
pub mod single_core_cell;
pub mod time_driver;
pub mod window;

pub mod bindings {
    #![allow(warnings)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    crate::info!("Main called!");

    // unsafe {
    //     let w = bindings::window_create();
    //     bindings::window_set_background_color(w, bindings::GColor8 { argb: 0b11110011 });
    //     bindings::window_stack_push(w, true);
    // }

    executor::init();
    executor::run(init);

    crate::info!("Main leaving!");
}

#[embassy_executor::task]
async fn async_main() {
    crate::info!("Async main called!");
    window::with_window(async |h| {
        core::future::pending::<()>().await;
    })
    .await
    .unwrap();
}

fn init(s: embassy_executor::Spawner) {
    crate::info!("Init called!");

    s.spawn(async_main().unwrap());
}

// extern, no_mangle so we can set a breakpoint
#[inline(never)]
#[unsafe(no_mangle)]
extern "C" fn trigger_panic() -> ! {
    unsafe {
        bindings::exit_reason_set(bindings::AppExitReason::APP_EXIT_NOT_SPECIFIED);
        bindings::window_stack_pop_all(false);

        // bindings::app_event_loop();
    };

    unsafe {
        let crash: *mut u32 = core::ptr::null_mut();
        core::ptr::write_volatile(crash, 0xDEADBEEF);
    }

    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let msg = info.message().as_str().unwrap_or("<no message>");
    crate::error!("Panic! {}", msg);
    trigger_panic();
}
