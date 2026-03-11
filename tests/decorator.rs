use sutils::FnWrap;

macro_rules! is_same {
    ($wrap:ident,$origin:ident,$ns:tt) => {
        is_same!($wrap, $ns :: $origin);
    };
    ($wrap:ident,$($origin:tt)+) => {
        #[allow(unused)]
        fn $wrap(input: i8) -> bool {
            let output: i8 = $($origin)* (input.into()).into();
            output == input
        }
    };
}

#[FnWrap(is_same)]
fn is_positive(input: i32) -> bool {
    input > 0
}

#[FnWrap(is_same)]
fn is_odd(input: i8) -> bool {
    input % 2 == 1
}

struct Foo {
    i32: i32,
}

impl Foo {
    #[FnWrap(is_same, Self)]
    fn is_negative(self) -> bool {
        self.i32 < 0
    }
}

impl From<i8> for Foo {
    fn from(value: i8) -> Self {
        Self { i32: value as i32 }
    }
}

#[test]
fn testing_ambiguous_decorator() {
    assert!(is_positive(1));
    assert!(is_odd(0));
    assert!(Foo::is_negative(-1));
    
    assert!(!is_positive(2));
    assert!(!is_odd(2));
    assert!(!Foo::is_negative(0));
}
