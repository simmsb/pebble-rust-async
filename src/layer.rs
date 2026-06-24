use core::ptr::NonNull;

use crate::bindings::{self, GRect};

// TODO: LayerRef/LayerMut might be completely wrong here. It might not make any
// sense to have the separation.

pub mod layer;
pub mod text;

pub use layer::{Layer, LayerRef, LayerMut};
pub use text::TextLayer;

pub trait IsLayer {
    fn layer<'a>(&'a self) -> LayerRef<'a>;
    fn layer_mut<'a>(&'a mut self) -> LayerMut<'a>;
}

impl<'a> IsLayer for Layer<'a> {
    fn layer(&self) -> LayerRef<'a> {
        LayerRef::from_ptr(self.inner)
    }

    fn layer_mut(&mut self) -> LayerMut<'a> {
        LayerMut::from_ptr(self.inner)
    }
}

impl<'a> IsLayer for TextLayer<'a> {
    fn layer(&self) -> LayerRef<'a> {
        let ptr = unsafe { bindings::text_layer_get_layer(self.inner.as_ptr()) };
        LayerRef::from_ptr(NonNull::new(ptr).unwrap())
    }

    fn layer_mut(&mut self) -> LayerMut<'a> {
        let ptr = unsafe { bindings::text_layer_get_layer(self.inner.as_ptr()) };
        LayerMut::from_ptr(NonNull::new(ptr).unwrap())
    }
}

pub trait AsChildLayer<'a>: Sized + IsLayer {
    fn new_unparented(frame: GRect) -> Option<Self>;
}

impl<'a> AsChildLayer<'a> for Layer<'a> {
    fn new_unparented(frame: GRect) -> Option<Self> {
        Self::new(frame)
    }
}

impl<'a> AsChildLayer<'a> for TextLayer<'a> {
    fn new_unparented(frame: GRect) -> Option<Self> {
        Self::new(frame)
    }
}
