use std::sync::MutexGuard;

pub use sutils_macro::Singleton;

use crate::ChainedStruct;

pub trait Singleton {
    fn One<'r>() -> &'r mut Self;
    fn OneSafe<'r>() -> SideLock<'r, Self>;
}

pub type SideLock<'r, T> = ChainedStruct<MutexGuard<'r, ()>, &'r mut T>;

pub trait SideGuard<'l, Inner>
where
    Self: 'l,
{
    fn side_guard<T>(self, data: &'l mut T) -> ChainedStruct<MutexGuard<'l, Inner>, &'l mut T>;
}

impl<'l, Inner> SideGuard<'l, Inner> for MutexGuard<'l, Inner> {
    fn side_guard<T>(self, data: &'l mut T) -> ChainedStruct<MutexGuard<'l, Inner>, &'l mut T> {
        let (prev, this) = (self, data);
        ChainedStruct { prev, this }
    }
}
