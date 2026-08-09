use std::ops::Deref;

use super::unsafe_ref::UnsafeRef;

pub struct LeakBox<T: ?Sized> {
    pub ptr: UnsafeRef<T>,
}

impl<T: ?Sized> Clone for LeakBox<T> {
    fn clone(&self) -> Self {
        Self {
            ptr: UnsafeRef::new(self.get_mut()),
        }
    }
}
impl<T: ?Sized> Copy for LeakBox<T> {}

impl<T: ?Sized> LeakBox<T> {
    pub fn new() -> Self {
        Self {
            ptr: UnsafeRef::new(UnsafeRef::null_unsized()),
        }
    }

    pub fn set_pointer(mut self, ptr: *mut T) -> Self {
        self.ptr = UnsafeRef::new(ptr as *const T);
        self
    }

    pub fn get_mut(&self) -> &mut T {
        self.ptr.must_mut()
    }

    pub fn into_box(self) -> Box<T> {
        unsafe { Box::from_raw(self.get_mut() as *mut T) }
    }

    pub fn drop(self) {
        if self.ptr.0 == [0; _] {
            return;
        }
        drop(self.into_box())
    }

    pub fn guarded(self) -> LeakBoxGuard<T> {
        LeakBoxGuard { ptr: self }
    }

    pub fn cast_to<Other: ?Sized>(self) -> LeakBox<Other> {
        LeakBox {
            ptr: self.ptr.assert(),
        }
    }
}

impl<T: ?Sized> From<Box<T>> for LeakBox<T> {
    fn from(value: Box<T>) -> Self {
        Self::new().set_pointer(Box::into_raw(value))
    }
}

impl<T> Deref for LeakBox<T> {
    type Target = UnsafeRef<T>;

    fn deref(&self) -> &Self::Target {
        &self.ptr
    }
}

pub struct LeakBoxGuard<T: ?Sized> {
    pub ptr: LeakBox<T>,
}

impl<T: ?Sized> Drop for LeakBoxGuard<T> {
    fn drop(&mut self) {
        self.ptr.drop();
    }
}
