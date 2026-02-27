use std::{
    ops::{Deref, DerefMut},
    sync::MutexGuard,
};

pub use sutils_macro::Singleton;

pub trait Singleton {
    fn One<'r>() -> &'r mut Self;
    fn OneSafe<'r>() -> SideGuardStruct<'r, Self>;
}

pub trait SideGuard<'l>
where
    Self: 'l,
{
    fn side_guard<T>(self, data: &'l mut T) -> SideGuardStruct<'l, T>;
}

impl<'l> SideGuard<'l> for MutexGuard<'l, ()> {
    fn side_guard<T>(self, data: &'l mut T) -> SideGuardStruct<'l, T> {
        SideGuardStruct::new(self, data)
    }
}

pub struct SideGuardStruct<'l, T>
where
    T: ?Sized,
{
    pub guard: MutexGuard<'l, ()>,
    data: &'l mut T,
}

impl<'l, T> SideGuardStruct<'l, T> {
    pub fn new(guard: MutexGuard<'l, ()>, data: &'l mut T) -> Self {
        Self { guard, data }
    }
}

impl<'l, T> Deref for SideGuardStruct<'l, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl<'l, T> DerefMut for SideGuardStruct<'l, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data
    }
}