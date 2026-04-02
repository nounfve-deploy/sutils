use std::{marker::PhantomData, ptr};

#[derive(Clone, Copy)]
pub struct LeakBox<T: ?Sized> {
    ptr: [u8; 16],
    inner: PhantomData<T>,
}

impl<T: ?Sized> LeakBox<T> {
    pub fn new() -> Self {
        Self {
            ptr: [0; _],
            inner: PhantomData,
        }
    }

    pub fn set_pointer(mut self, ptr: *mut T) -> Self {
        assert!(size_of::<*mut T>() <= size_of::<Self>());
        self.ptr = unsafe { ptr::read(&ptr as *const *mut T as *const [u8; _]) };
        self
    }

    pub fn get_mut(&self) -> &mut T {
        let ptr = unsafe { ptr::read(&self.ptr as *const [u8; _] as *const *mut T) };
        unsafe { &mut *ptr }
    }

    pub fn into_box(self) -> Box<T> {
        unsafe { Box::from_raw(self.get_mut() as *mut T) }
    }

    pub fn drop(self) {
        if self.ptr == [0; _] {
            return;
        }
        drop(self.into_box())
    }
    
    pub fn cast_to<Other: ?Sized>(self) -> LeakBox<Other> {
        LeakBox {
            ptr: self.ptr,
            inner: PhantomData,
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
