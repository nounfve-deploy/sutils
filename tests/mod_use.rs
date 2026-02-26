mod singleton;

mod inner {
    pub struct Bar {
        pub u8: u8,
    }
}

#[sutils::mod_use_all]
struct UseAll;

// test redefine ident
#[allow(dead_code)]
struct UseAll {
    m: u8,
}

#[test]
fn test_unused_ident() {
    let _x = Bar { u8: 1 };
    let _y = FooSingleton::default();
}
