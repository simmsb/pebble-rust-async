use core::{
    marker::{PhantomData, PhantomPinned},
    ops::{Deref, DerefMut}, ptr::NonNull,
};
use pin_init::{PinInit, pin_init};

use super::AsChildLayer;
use crate::{bindings::{self, GRect}, graphics_context::GContext};

pub(crate) unsafe extern "C" fn layer_callback(
    layer: *mut bindings::Layer,
    context: *mut bindings::GContext,
) {
    let layer = LayerMut::from_ptr(NonNull::new(layer).unwrap());

    let context = GContext::new(NonNull::new(context).unwrap());

    let cb = unsafe {
        bindings::layer_get_data(layer.inner.inner.as_ptr()) as *mut LayerUpdateProcVTable
    };

    unsafe {
        (**cb)(layer, context);
    }
}

/// As [Layer], but isn't owned and therefore doesn't destroy the layer on drop.
pub struct LayerRef<'a> {
    pub(crate) inner: core::mem::ManuallyDrop<Layer<'a>>,
}

impl<'a> LayerRef<'a> {
    pub(crate) fn from_ptr(ptr: NonNull<bindings::Layer>) -> Self {
        Self {
            inner: core::mem::ManuallyDrop::new(Layer::from_ptr(ptr)),
        }
    }
}

impl<'a> Deref for LayerRef<'a> {
    type Target = Layer<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// As [Layer], but isn't owned and therefore doesn't destroy the layer on drop.
pub struct LayerMut<'a> {
    pub(crate) inner: core::mem::ManuallyDrop<Layer<'a>>,
}

impl<'a> LayerMut<'a> {
    pub(crate) fn from_ptr(ptr: NonNull<bindings::Layer>) -> Self {
        Self {
            inner: core::mem::ManuallyDrop::new(Layer::from_ptr(ptr)),
        }
    }
}

impl<'a> Deref for LayerMut<'a> {
    type Target = Layer<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a> DerefMut for LayerMut<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

/// A layer. The lifetime is used to track children.
pub struct Layer<'a> {
    pub(crate) inner: NonNull<bindings::Layer>,

    pub(crate) _phantom: PhantomData<&'a ()>,
}

/// A layer with an attached update function. This needs to be pinned in order
/// to have a stable reference to the callback data.
#[pin_init::pin_data]
pub struct LayerWithUpdateProc<'a, F> {
    inner: Layer<'a>,

    callback: F,

    #[pin]
    _pin_phantom: PhantomPinned,
}

impl<'a, F> Deref for LayerWithUpdateProc<'a, F> {
    type Target = Layer<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a, F> DerefMut for LayerWithUpdateProc<'a, F> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

pub(crate) type LayerUpdateProcVTable =
    *mut (dyn for<'cb> FnMut(LayerMut<'cb>, GContext<'cb>) + 'static);

impl<'a> Layer<'a> {
    pub(crate) fn new(frame: GRect) -> Option<Self> {
        let ptr =
            unsafe { bindings::layer_create_with_data(frame, size_of::<LayerUpdateProcVTable>()) };
        NonNull::new(ptr).map(Self::from_ptr)
    }

    pub(crate) fn from_ptr(ptr: NonNull<bindings::Layer>) -> Self {
        Self {
            inner: ptr,
            _phantom: PhantomData,
        }
    }

    pub fn new_child<'child: 'a, LayerT>(&self, frame: GRect) -> Option<LayerT>
    where
        LayerT: AsChildLayer<'child>,
    {
        let child = LayerT::new_unparented(frame)?;
        unsafe {
            bindings::layer_add_child(self.inner.as_ptr(), child.layer().inner.inner.as_ptr());
        }
        Some(child)
    }

    pub fn mark_dirty(&mut self) {
        unsafe {
            bindings::layer_mark_dirty(self.inner.as_ptr());
        }
    }

    pub fn frame(&self) -> GRect {
        unsafe { bindings::layer_get_frame(self.inner.as_ptr()) }
    }

    pub fn set_frame(&mut self, frame: GRect) {
        unsafe {
            bindings::layer_set_frame(self.inner.as_ptr(), frame);
        }
    }

    pub fn bounds(&self) -> GRect {
        unsafe { bindings::layer_get_bounds(self.inner.as_ptr()) }
    }

    pub fn set_bounds(&mut self, bounds: GRect) {
        unsafe {
            bindings::layer_set_bounds(self.inner.as_ptr(), bounds);
        }
    }

    /// Attach an update proc to this layer.
    ///
    /// This returns a [PinInit] as we need
    /// to pass the pebble SDK a pointer to
    /// the closure passed in, if
    /// [LayerWithUpdateProc] could move, it
    /// would invalidate this reference.
    pub fn with_update_proc<F>(self, callback: F) -> impl PinInit<LayerWithUpdateProc<'a, F>>
    where
        F: for<'cb> FnMut(LayerMut<'cb>, GContext<'cb>) + 'a,
    {
        pin_init!(LayerWithUpdateProc {
            inner: self,
            callback,
            _pin_phantom: PhantomPinned,
        })
        .pin_chain(|p| {
            unsafe {
                let project = p.project();

                let callback_vtable = project.callback
                    as *mut (dyn for<'cb> FnMut(LayerMut<'cb>, GContext<'cb>) + 'a);

                // N.B. this erases the lifetimes of the closure captures
                let callback_vtable_static =
                    core::mem::transmute::<_, LayerUpdateProcVTable>(callback_vtable);

                // Pointer to the fat dyn pointer
                let cb = bindings::layer_get_data(project.inner.inner.as_ptr()) as *mut LayerUpdateProcVTable;
                cb.write(callback_vtable_static);

                bindings::layer_set_update_proc(project.inner.inner.as_ptr(), Some(layer_callback));
            }

            Ok(())
        })
    }
}

impl<'a> Drop for Layer<'a> {
    fn drop(&mut self) {
        unsafe {
            bindings::layer_destroy(self.inner.as_ptr());
        }
    }
}
