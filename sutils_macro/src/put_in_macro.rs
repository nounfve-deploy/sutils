use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Token};

use crate::ext::tuple_parse::parse_tuple;

pub fn put_in_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let (is_ref, wrap_macro, rest) = parse_tuple!(attr, Option<Token![ref]>, Ident, TokenStream);

    let item_origin = is_ref.map(|_| quote! {#item});

    let item = quote! {
        #item_origin
        #wrap_macro ! { #item #rest}
    };
    item
}
