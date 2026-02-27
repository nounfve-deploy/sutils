use std::thread;

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
    assert_eq!(one.u8, 6);

    let t1 = thread::spawn(|| {
        FooSingleton::OneSafe().apply(|it| it.u8 *= 4);
    });
    let t2 = thread::spawn(|| {
        FooSingleton::OneSafe().apply(|it| it.u8 *= 5);
    });
    _ = (t1.join(), t2.join());

    assert_eq!(one.u8, 120);
}
