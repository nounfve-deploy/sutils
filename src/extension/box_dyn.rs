use crate::{IntoLifeTime, UnsafeMutTrait, UnsafeRefTrait};

pub trait BoxDynExt {
    fn assert<T>(&self) -> &T;
    fn assert_mut<T>(&mut self) -> &mut T;
}

impl<Dyn: ?Sized> BoxDynExt for Box<Dyn> {
    fn assert<T>(&self) -> &T {
        let refer = self.as_ref().unsafe_ref();
        refer.assert::<T>().must_mut().into_lifetime()
    }

    fn assert_mut<T>(&mut self) -> &mut T {
        let refer = self.as_mut().unsafe_mut();
        refer.assert::<T>().must_mut().into_lifetime()
    }
}
