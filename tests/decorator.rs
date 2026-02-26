use sutils::FnWrap;

macro_rules! is_same {
    () => {
        pub fn __wrapper__(input: i8) -> bool {
            let output: i8 = __inner__(input.into()).into();
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

#[test]
fn testing_ambiguous_decorator() {
    assert!(is_positive(1));
    assert!(is_odd(0));
    
    assert!(!is_positive(2));
    assert!(!is_odd(2));
}
