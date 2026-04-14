use super::unsafe_ref::UnsafeRef;

#[derive(Clone, Copy)]
pub struct LeakBox<T: ?Sized> {
    refer: UnsafeRef<T>,
}

impl<T: ?Sized> LeakBox<T> {
    pub fn new() -> Self {
        Self {
            refer: UnsafeRef::new(UnsafeRef::null_unsized()),
        }
    }

    pub fn set_pointer(mut self, ptr: *mut T) -> Self {
        self.refer = UnsafeRef::new(ptr as *const T);
        self
    }

    pub fn get_mut(&self) -> &mut T {
        self.refer.must_mut()
    }

    pub fn into_box(self) -> Box<T> {
        unsafe { Box::from_raw(self.get_mut() as *mut T) }
    }

    pub fn drop(self) {
        if self.refer.0 == [0; _] {
            return;
        }
        drop(self.into_box())
    }

    pub fn cast_to<Other: ?Sized>(self) -> LeakBox<Other> {
        LeakBox {
            refer: self.refer.assert(),
        }
    }
}

impl<T: ?Sized> From<Box<T>> for LeakBox<T> {
    fn from(value: Box<T>) -> Self {
        Self::new().set_pointer(Box::into_raw(value))
    }
}

unsafe impl<T: ?Sized> Send for LeakBox<T> {}
unsafe impl<T: ?Sized> Sync for LeakBox<T> {}
