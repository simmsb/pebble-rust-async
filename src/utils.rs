use crate::bindings::{self, AppLaunchReason};

pub fn launch_reason() -> AppLaunchReason {
    unsafe { bindings::launch_reason() }
}
