use std::borrow::Borrow;

use sutils_macro::ExternImpl;

struct Foo<T = ()> {
    val: T,
}

#[ExternImpl]
impl<T: Default + Sized> Foo<T>
where
    T: PartialEq + Eq + Default,
{
    pub(crate) fn equal<R: Borrow<T>>(&self, other: R) -> Option<T> {
        if self.val == *other.borrow() {
            return Some(T::default());
        }
        None
    }

    fn notx(self) -> T {
        todo!()
    }
}

#[test]
fn extern_impl_test() {
    let x = Foo { val: 1 };
    FooEmpl::equal(&x, 1);
    assert!(x.equal(1).is_some());
    assert!(FooEmpl::equal(&x, 1).is_some());
}
