#[allow(nonstandard_style)]
pub trait IntoResult
where
    Self: Sized,
{
    fn Ok<E>(self) -> Result<Self, E>;
    fn Err<O>(self) -> Result<O, Self>;
}

impl<T> IntoResult for T {
    fn Ok<E>(self) -> Result<Self, E> {
        Ok(self)
    }

    fn Err<O>(self) -> Result<O, Self> {
        Err(self)
    }
}

#[allow(nonstandard_style)]
pub trait IntoOption
where
    Self: Sized,
{
    fn Some(self) -> Option<Self>;
    fn None(&self) -> Option<Self>;
}

impl<T> IntoOption for T {
    fn Some(self) -> Option<Self> {
        Some(self)
    }

    fn None(&self) -> Option<Self> {
        None
    }
}
