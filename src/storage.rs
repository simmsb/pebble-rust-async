use core::mem::MaybeUninit;

use crate::bindings;

pub fn exists(key: u32) -> bool {
    unsafe { bindings::persist_exists(key) }
}

pub fn get_size(key: u32) -> i32 {
    unsafe { bindings::persist_get_size(key) }
}

pub fn read(key: u32, buffer: &mut [u8]) -> Result<usize, bindings::StatusCode> {
    collect_error(unsafe {
        bindings::persist_read_data(key, buffer.as_mut_ptr() as *mut _, buffer.len())
    })
}

pub fn read_uninit(
    key: u32,
    buffer: &mut [MaybeUninit<u8>],
) -> Result<usize, bindings::StatusCode> {
    collect_error(unsafe {
        bindings::persist_read_data(key, buffer.as_mut_ptr().cast(), buffer.len())
    })
}

pub fn write(key: u32, data: &[u8]) -> Result<usize, bindings::StatusCode> {
    collect_error(unsafe { bindings::persist_write_data(key, data.as_ptr() as *mut _, data.len()) })
}

fn collect_error(val: core::ffi::c_int) -> Result<usize, bindings::StatusCode> {
    if val < 0 {
        Err(unsafe { core::mem::transmute(val) })
    } else {
        Ok(val as usize)
    }
}
