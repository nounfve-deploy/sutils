pub use sutils_macro::Singleton;

pub trait Singleton {
    fn One<'r>() -> &'r mut Self;
}
