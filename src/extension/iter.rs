use crate::IntoLifeTime;

pub trait IterDeref<'a, 'b, T> {
    fn deref_mut(self) -> impl Iterator<Item = &'b mut T>
    where
        Self: Iterator<Item = &'a mut &'b mut T>,
        T: 'b,
        'b: 'a;
}

impl<'a, 'b, T, Iter> IterDeref<'a, 'b, T> for Iter {
    fn deref_mut(self) -> impl Iterator<Item = &'b mut T>
    where
        Self: Iterator<Item = &'a mut &'b mut T>,
        T: 'b,
        'b: 'a,
    {
        self.map(|x| (*x).into_lifetime())
    }
}
