use proc_macro::TokenStream;

mod singleton;
use crate::singleton::singleton_macro;

#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn Singleton(attr: TokenStream, item: TokenStream) -> TokenStream {
    singleton_macro(attr.into(), item.into()).into()
}
