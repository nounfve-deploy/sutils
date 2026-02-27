use std::ops::{Deref, DerefMut};

pub struct ChainedStruct<P, T>
where
    T: ?Sized,
{
    pub prev: P,
    pub this: T,
}

impl<P, T> Deref for ChainedStruct<P, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.this
    }
}

impl<P, T> DerefMut for ChainedStruct<P, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.this
    }
}
