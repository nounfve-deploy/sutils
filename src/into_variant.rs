#[allow(nonstandard_style)]
pub trait IntoResult<T>
where
    Self: Sized,
{
    fn Ok<E>(self) -> Result<T, E>;
    fn Err<O>(self) -> Result<O, T>;
    fn ErrVoid(self) -> Result<(), T>;
}

impl<T, This> IntoResult<T> for This
where
    This: Into<T>,
{
    fn Ok<E>(self) -> Result<T, E> {
        Ok(self.into())
    }

    fn Err<O>(self) -> Result<O, T> {
        Err(self.into())
    }

    fn ErrVoid(self) -> Result<(), T> {
        Err(self.into())
    }
}

#[allow(nonstandard_style)]
pub trait IntoOption
where
    Self: Sized,
{
    fn Some(self) -> Option<Self>;
    fn None(&self) -> Option<Self>;
    fn Some_if(self, f: impl FnOnce(&Self) -> bool) -> Option<Self>;
}

impl<This> IntoOption for This {
    fn Some(self) -> Option<Self> {
        Some(self)
    }

    fn None(&self) -> Option<Self> {
        None
    }

    fn Some_if(self, f: impl FnOnce(&Self) -> bool) -> Option<Self> {
        if f(&self) { Some(self) } else { None }
    }
}

pub trait IntoBox {
    fn boxed(self) -> Box<Self>;
}

impl<T> IntoBox for T {
    fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}
