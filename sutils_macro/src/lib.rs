use proc_macro::TokenStream;

mod fn_wrap;
mod mod_use_all;
mod singleton;
use crate::{fn_wrap::fn_wrap_macro, mod_use_all::mod_use_all_macro, singleton::singleton_macro};

macro_rules! macro_debug {
    ($token:expr) => {{
        let token: TokenStream = $token;
        #[cfg(feature = "macro_debug")]
        println!("{}", token.to_string());
        token
    }};
}

#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn Singleton(attr: TokenStream, item: TokenStream) -> TokenStream {
    macro_debug!(singleton_macro(attr.into(), item.into()).into())
}

#[proc_macro_attribute]
pub fn mod_use_all(attr: TokenStream, item: TokenStream) -> TokenStream {
    macro_debug!(mod_use_all_macro(attr.into(), item.into()).into())
}

#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn FnWrap(attr: TokenStream, item: TokenStream) -> TokenStream {
    macro_debug!(fn_wrap_macro(attr.into(), item.into()).into())
}
