pub trait ContextFunction<R>
where
    Self: Sized,
{
    fn apply(self, f: impl FnOnce(&mut Self) -> R) -> Self;
    fn run(self, f: impl FnOnce(Self) -> R) -> R;
    fn when<Other>(self, other: Other, f: impl FnOnce(&mut Self) -> R) -> Self
    where
        Other: PartialEq<Self>;
    fn whenMap<Other>(self, other: Other, f: impl FnOnce(&mut Self) -> R) -> Option<R>
    where
        Other: PartialEq<Self>;
    fn apply_foreach<E>(self, f: impl FnMut(&mut E) -> R) -> impl Iterator<Item = E>
    where
        Self: Iterator<Item = E>;
    fn apply_first<E>(self, f: impl FnMut(&mut E) -> R) -> impl Iterator<Item = E>
    where
        Self: Iterator<Item = E>;
}

impl<T, R> ContextFunction<R> for T {
    fn apply(mut self, f: impl FnOnce(&mut Self) -> R) -> Self {
        f(&mut self);
        self
    }

    fn run(self, f: impl FnOnce(Self) -> R) -> R {
        f(self)
    }

    fn when<Other>(mut self, other: Other, f: impl FnOnce(&mut Self) -> R) -> Self
    where
        Other: PartialEq<Self>,
    {
        if other == self {
            f(&mut self);
        };
        self
    }

    fn whenMap<Other>(mut self, other: Other, f: impl FnOnce(&mut Self) -> R) -> Option<R>
    where
        Other: PartialEq<Self>,
    {
        if other == self {
            Some(f(&mut self))
        } else {
            None
        }
    }

    fn apply_foreach<E>(self, mut f: impl FnMut(&mut E) -> R) -> impl Iterator<Item = E>
    where
        Self: Iterator<Item = E>,
    {
        self.map(move |mut e| {
            f(&mut e);
            e
        })
    }

    fn apply_first<E>(self, mut f: impl FnMut(&mut E) -> R) -> impl Iterator<Item = E>
    where
        Self: Iterator<Item = E>,
    {
        let mut run = true;
        self.map(move |mut e| {
            run.when(true, |_| {
                run = false;
                f(&mut e);
            });
            e
        })
    }
}

pub trait PartialEqEx<T>
where
    T: PartialEq<Self>,
{
    fn Not(self) -> NotSelf<T>;
}

impl<T> PartialEqEx<T> for T
where
    T: PartialEq<Self>,
{
    fn Not(self) -> NotSelf<T> {
        NotSelf { inner: self }
    }
}

pub struct NotSelf<T> {
    inner: T,
}

impl<T, Other> PartialEq<Other> for NotSelf<T>
where
    T: PartialEq<Other>,
{
    fn eq(&self, other: &Other) -> bool {
        !(&self.inner == other)
    }
}

pub trait TypedInto
where
    Self: Sized,
{
    fn inTo<I>(self) -> I
    where
        Self: Into<I>;
}

impl<T> TypedInto for T {
    fn inTo<I>(self) -> I
    where
        Self: Into<I>,
    {
        Into::into(self)
    }
}

pub trait IntoFnMut<A1, R> {
    fn into_fn_mut(self) -> impl FnMut(A1) -> R;
}

impl<T, A1, R> IntoFnMut<A1, R> for T
where
    T: FnMut() -> R,
{
    fn into_fn_mut(mut self) -> impl FnMut(A1) -> R {
        move |_| self()
    }
}
