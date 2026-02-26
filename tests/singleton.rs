use sutils::{ContextFunction, Singleton};

#[Singleton]
pub struct FooSingleton {
    u8: u8,
}

impl Default for FooSingleton {
    fn default() -> Self {
        Self { u8: 2 }
    }
}

#[test]
fn test_singleton() {
    FooSingleton::One().run(|it| {
        it.u8 *= 3;
    });
    let one = FooSingleton::One();
    assert_eq!(one.u8, 6)
}
