use std::mem;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemFn, Token, Type, Visibility, parse::Parser, parse2, punctuated::Punctuated};

pub fn fn_wrap_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut inner = parse2::<ItemFn>(item).expect("parse inner function failed");
    let mut attr = <Punctuated<Type, Token![,]>>::parse_terminated
        .parse2(attr.clone())
        .expect("parse decor failed")
        .into_iter();
    let (wrap_macro, rest) = {
        let first = attr.next().unwrap();
        let rest = attr.collect::<Punctuated<_, Token![,]>>();
        let rest = if rest.is_empty() {
            None
        } else {
            Some(quote! {
                ,#rest
            })
        };
        (first, rest)
    };

    let inner_ident = format_ident!("{}__inner__", inner.sig.ident);
    let _out_vis = mem::replace(&mut inner.vis, Visibility::Inherited);
    let out_ident = mem::replace(&mut inner.sig.ident, inner_ident.clone());

    let submod = quote! {
        #wrap_macro ! (#out_ident,#inner_ident #rest);
        #[allow(non_snake_case)]
        #inner
    };
    submod
}
