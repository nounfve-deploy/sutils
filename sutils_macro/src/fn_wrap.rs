use std::mem;

use crate::ext::punctuated::PunctuatedExt;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemFn, Token, Type, Visibility, parse2, punctuated::Punctuated};

pub fn fn_wrap_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut inner = parse2::<ItemFn>(item).expect("parse inner function failed");
    let attr = <Punctuated<Type, Token![,]>>::parse2(attr.clone()).expect("parse decor failed");
    let (wrap_macro, rest) = {
        let (first, rest) = attr.pop_first();
        (first.unwrap(), rest.map(|rest| quote! {, #rest}))
    };

    let inner_ident = format_ident!("{}__inner__", inner.sig.ident);
    let _out_vis = mem::replace(&mut inner.vis, Visibility::Inherited);
    let out_ident = mem::replace(&mut inner.sig.ident, inner_ident.clone());

    let output = quote! {
        #wrap_macro ! (#out_ident,#inner_ident #rest);
        #[allow(non_snake_case)]
        #inner
    };
    output
}
