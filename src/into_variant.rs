#[allow(nonstandard_style)]
pub trait IntoResult<T>
where
    Self: Sized,
{
    fn Ok<E>(self) -> Result<T, E>;
    fn Err<O>(self) -> Result<O, T>;
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
}

#[allow(nonstandard_style)]
pub trait IntoOption
where
    Self: Sized,
{
    fn Some(self) -> Option<Self>;
    fn None(&self) -> Option<Self>;
}

impl<This> IntoOption for This
{
    fn Some(self) -> Option<Self> {
        Some(self)
    }

    fn None(&self) -> Option<Self> {
        None
    }
}
