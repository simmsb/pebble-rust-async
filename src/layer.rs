use core::ptr::NonNull;

use crate::bindings::{self, GRect};

extern "C" fn layer_callback()

pub struct Layer<'a> {
    inner: NonNull<bindings::Layer>,
}

impl<'a> Layer<'a> {
    fn new(frame: GRect) -> Option<Self> {
        let ptr = unsafe { bindings::layer_create(frame) };
        NonNull::new(ptr).map(|p| Layer { inner: p })
    }

    pub fn new_child<'child: 'a>(&self, frame: GRect) -> Option<Layer<'child>> {
        let child = Layer::new(frame)?;
        unsafe {
            bindings::layer_add_child(self.inner.as_ptr(), child.inner.as_ptr());
        }
        Some(child)
    }

    pub fn mark_dirty(&mut self) {
        unsafe {
            bindings::layer_mark_dirty(self.inner.as_ptr());
        }
    }

    pub fn with_callback(self)
}

impl<'a> Drop for Layer<'a> {
    fn drop(&mut self) {
        unsafe {
            bindings::layer_destroy(self.inner.as_ptr());
        }
    }
}
