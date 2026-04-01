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
pub trait IntoOption<T>
where
    Self: Sized,
{
    fn Some(self) -> Option<T>;
    fn None(&self) -> Option<T>;
}

impl<T, This> IntoOption<T> for This
where
    This: Into<T>,
{
    fn Some(self) -> Option<T> {
        Some(self.into())
    }

    fn None(&self) -> Option<T> {
        None
    }
}
