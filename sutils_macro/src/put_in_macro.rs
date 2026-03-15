use proc_macro2::TokenStream;
use quote::quote;
use syn::{Token, Type, punctuated::Punctuated};

use crate::ext::punctuated::PunctuatedExt;

pub fn put_in_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = <Punctuated<Type, Token![,]>>::parse2(attr.clone()).expect("parse decor failed");
    let (wrap_macro, rest) = {
        let (first, rest) = attr.pop_first();
        (first.unwrap(), rest.map(|rest| quote! {, #rest}))
    };

    let item = quote! {
        #wrap_macro ! { #item #rest}
    };
    item
}
