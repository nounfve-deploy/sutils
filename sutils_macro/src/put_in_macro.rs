use std::str::FromStr;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn put_in_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = attr.to_string();
    let (wrap_macro, rest) = match attr.split_once(",") {
        Some((first, rest)) => (
            first.to_string(),
            Some(TokenStream::from_str(rest).unwrap()),
        ),
        None => (attr, None),
    };

    let rest = rest.map(|r| quote!(,#r));

    let mut item_origin = None;
    let wrap_macro = if let Some(ident) = wrap_macro.strip_prefix("ref ") {
        item_origin = Some(quote! {#item});
        format_ident!("{}", ident.trim())
    } else {
        format_ident!("{}", wrap_macro.trim())
    };

    let item = quote! {
        #item_origin
        #wrap_macro ! { #item #rest}
    };
    item
}
