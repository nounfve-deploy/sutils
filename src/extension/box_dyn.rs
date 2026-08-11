use crate::{IntoLifeTime, UnsafeMutTrait, UnsafeRefTrait};

#[deprecated="use DynAssert"]
pub trait BoxDynExt {
    fn assert<T>(&self) -> &T;
    fn assert_mut<T>(&mut self) -> &mut T;
}

#[allow(deprecated)]
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

pub trait DynAssert<Dyn>
where
    Dyn: ?Sized,
{
    fn assert<T>(&self) -> &T;
    fn assert_mut<T>(&mut self) -> &mut T;
}

impl<Ref, Dyn> DynAssert<Dyn> for Ref
where
    Dyn: ?Sized,
    Self: AsRef<Dyn>,
    Self: AsMut<Dyn>,
{
    fn assert<T>(&self) -> &T {
        let refer = self.as_ref().unsafe_ref();
        refer.assert::<T>().must_mut().into_lifetime()
    }

    fn assert_mut<T>(&mut self) -> &mut T {
        let refer = self.as_mut().unsafe_mut();
        refer.assert::<T>().must_mut().into_lifetime()
    }
}

