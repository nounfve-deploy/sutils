use proc_macro::TokenStream;

mod mod_use_all;
mod singleton;
mod fn_wrap;
use crate::{fn_wrap::fn_wrap_macro, mod_use_all::mod_use_all_macro, singleton::singleton_macro};

#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn Singleton(attr: TokenStream, item: TokenStream) -> TokenStream {
    singleton_macro(attr.into(), item.into()).into()
}

#[proc_macro_attribute]
pub fn mod_use_all(attr: TokenStream, item: TokenStream) -> TokenStream {
    mod_use_all_macro(attr.into(), item.into()).into()
}

#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn FnWrap(attr: TokenStream, item: TokenStream) -> TokenStream {
    fn_wrap_macro(attr.into(), item.into()).into()
}