use std::{marker::PhantomData, ops::{Deref, DerefMut}};

pub trait UnsafeRefTrait<T> {
    fn unsafe_ref(self) -> UnsafeRef<T>;
}

pub trait UnsafeMutTrait<T> {
    fn unsafe_mut(self) -> UnsafeRef<T>;
}

impl<T> UnsafeRefTrait<T> for &T {
    fn unsafe_ref(self) -> UnsafeRef<T> {
        let ptr = self as *const T as usize;
        UnsafeRef(ptr, PhantomData)
    }
}

impl<T> UnsafeMutTrait<T> for &mut T {
    fn unsafe_mut(self) -> UnsafeRef<T> {
        let ptr = self as *const T as usize;
        UnsafeRef(ptr, PhantomData)
    }
}

pub struct UnsafeRef<T>(usize, PhantomData<T>);

impl<T> UnsafeRef<T> {
    pub fn must_mut(&self) -> &mut T {
        let ptr = self.0;
        unsafe { &mut *(ptr as *mut T) }
    }
}

impl<T> Deref for UnsafeRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let ptr = self.0;
        unsafe { &mut *(ptr as *mut T) }
    }
}

impl<T> DerefMut for UnsafeRef<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let ptr = self.0;
        unsafe { &mut *(ptr as *mut T) }
    }
}

impl<T> Clone for UnsafeRef<T> {
    fn clone(&self) -> Self {
        Self(self.0, self.1)
    }
}

impl<T> Copy for UnsafeRef<T> {}
