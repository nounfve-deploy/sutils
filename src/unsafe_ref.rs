use std::marker::PhantomData;

pub trait UnsafeRefTrait<T> {
    fn unsafe_ref(self) -> UnsafeRef<T>;
}

pub trait UnsafeMutTrait<T> {
    fn unsafe_mut(self) -> UnsafeMut<T>;
}

impl<T> UnsafeRefTrait<T> for &T {
    fn unsafe_ref(self) -> UnsafeRef<T> {
        let ptr = self as *const T as usize;
        UnsafeRef(ptr, PhantomData)
    }
}

impl<T> UnsafeMutTrait<T> for &mut T {
    fn unsafe_mut(self) -> UnsafeMut<T> {
        let ptr = self as *const T as usize;
        UnsafeMut(UnsafeRef(ptr, PhantomData))
    }
}

pub struct UnsafeRef<T>(usize, PhantomData<T>);
pub struct UnsafeMut<T>(UnsafeRef<T>);

impl<T> UnsafeMut<T> {
    pub fn must_mut(&self) -> &mut T {
        let ptr = self.0.0;
        unsafe { &mut *(ptr as *mut T) }
    }
}

impl<T> AsRef<T> for UnsafeRef<T> {
    fn as_ref(&self) -> &T {
        let ptr = self.0;
        unsafe { &*(ptr as *const T) }
    }
}

impl<T> AsMut<T> for UnsafeMut<T> {
    fn as_mut(&mut self) -> &mut T {
        self.must_mut()
    }
}

impl<T> Clone for UnsafeRef<T> {
    fn clone(&self) -> Self {
        Self(self.0, self.1)
    }
}

impl<T> Copy for UnsafeRef<T> {}

impl<T> Clone for UnsafeMut<T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<T> Copy for UnsafeMut<T> {}
