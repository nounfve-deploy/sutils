use proc_macro::TokenStream;

mod mod_use_all;
mod singleton;
use crate::{mod_use_all::mod_use_all_macro, singleton::singleton_macro};

#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn Singleton(attr: TokenStream, item: TokenStream) -> TokenStream {
    singleton_macro(attr.into(), item.into()).into()
}

#[proc_macro_attribute]
pub fn mod_use_all(attr: TokenStream, item: TokenStream) -> TokenStream {
    mod_use_all_macro(attr.into(), item.into()).into()
}
