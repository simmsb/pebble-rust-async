use core::cell::{Ref, RefCell, RefMut};

pub struct SingleCoreCell<T> {
    value: RefCell<T>,
}

impl<T> SingleCoreCell<T> {
    pub const fn new(value: T) -> Self {
        Self {
            value: RefCell::new(value),
        }
    }

    pub fn get<'a>(&'a self) -> Ref<'a, T> {
        self.value.borrow()
    }

    pub fn get_mut<'a>(&'a self) -> RefMut<'a, T> {
        self.value.borrow_mut()
    }
}

// pebble apps are single threaded and non-reentrant I hope?
unsafe impl<T> Send for SingleCoreCell<T> {}
unsafe impl<T> Sync for SingleCoreCell<T> {}
