
pub trait IntoLifeTime<T> {
    fn into_lifetime(self) -> T;
}

impl<'o, T> IntoLifeTime<&'o mut T> for &mut T {
    fn into_lifetime(self) -> &'o mut T {
        unsafe { &mut *(self as *mut T) }
    }
}

impl<'o, T> IntoLifeTime<&'o T> for &T {
    fn into_lifetime(self) -> &'o T {
        unsafe { &*(self as *const T) }
    }
}
