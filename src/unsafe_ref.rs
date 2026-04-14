use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

pub trait UnsafeRefTrait<T: ?Sized> {
    fn unsafe_ref(self) -> UnsafeRef<T>;
}

pub trait UnsafeMutTrait<T: ?Sized> {
    fn unsafe_mut(self) -> UnsafeRef<T>;
}

unsafe_ref_impl_for!(T);
unsafe_ref_impl_for!(Box<T>);

pub struct UnsafeRef<T: ?Sized>(
    // force breakline
    pub [u8; FAT_POINTER_SIZE],
    PhantomData<T>,
);
const FAT_POINTER_SIZE: usize = size_of::<[usize; 2]>();

impl<T: ?Sized> UnsafeRef<T> {
    pub const fn new(ptr: *const T) -> Self {
        assert!(size_of::<*const T>() <= size_of::<Self>());
        let ptr = unsafe { std::ptr::read(&ptr as *const *const T as *const [u8; _]) };
        Self(ptr, PhantomData)
    }

    pub fn assert<Other: ?Sized>(self) -> UnsafeRef<Other> {
        unsafe { std::mem::transmute(self) }
    }

    pub fn raw_mut(&self) -> *mut T {
        let ptr = &self.0;
        let ptr = unsafe { std::ptr::read(ptr as *const [u8; _] as *const *mut T) };
        ptr
    }

    pub fn must_mut(&self) -> &mut T {
        unsafe { &mut *(self.raw_mut()) }
    }

    pub const fn null_unsized() -> *mut T {
        let null = [0u8; FAT_POINTER_SIZE];
        let null = unsafe { std::ptr::read(&null as *const [u8; _] as *const *mut T) };
        null
    }
}

impl<T> Deref for UnsafeRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.must_mut()
    }
}

impl<T> DerefMut for UnsafeRef<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.must_mut()
    }
}

impl<T> Clone for UnsafeRef<T> {
    fn clone(&self) -> Self {
        Self(self.0, self.1)
    }
}

impl<T> Copy for UnsafeRef<T> {}

macro_rules! unsafe_ref_impl_for {
    ($Ref:ty) => {
        impl<T: ?Sized> UnsafeRefTrait<T> for &$Ref {
            fn unsafe_ref(self) -> UnsafeRef<T> {
                UnsafeRef::new(self as &T as *const T)
            }
        }

        impl<T: ?Sized> UnsafeMutTrait<T> for &mut $Ref {
            fn unsafe_mut(self) -> UnsafeRef<T> {
                UnsafeRef::new(self as &T as *const T)
            }
        }
    };
}
use unsafe_ref_impl_for;