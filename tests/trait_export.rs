use sutils::TraitExport;

trait FooTrait {
    fn fn_static(input: u8) -> u8;
    fn fn_ref(&self) -> u8;
    fn fn_mut_ref(&mut self, add: u8, multi: u8) -> u8;
}

struct Foo {
    u8: u8,
}

#[TraitExport(&mut Foo{u8:3})]
#[allow(non_snake_case)]
impl FooTrait for Foo {
    #![allow(non_camel_case_types)]
    fn fn_static(input: u8) -> u8 {
        input
    }

    fn fn_ref(&self) -> u8 {
        self.u8
    }

    fn fn_mut_ref(&mut self, add: u8, multi: u8) -> u8 {
        self.u8 += add;
        self.u8 *= multi;
        self.u8
    }
}

#[test]
fn test_trait_export() {
    let mut foo = Foo { u8: 1 };

    assert!(foo.fn_mut_ref(0, 2) == 2);
    assert!(foo.fn_mut_ref(0, 2) == 4);
    assert!(foo.fn_mut_ref(0, 2) == 8);

    assert!(fn_mut_ref(0, 2) == 6);
    assert!(fn_mut_ref(0, 2) == 6);

    assert!(Foo::fn_static(6) == fn_static(6));
    assert!(Foo::fn_static(7) == fn_static(7));
}
