use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, ImplItem, ImplItemFn, ItemImpl, PathSegment, Signature, Token, Type, parse2};

use crate::ext::tuple_parse::parse_tuple;

pub fn extern_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let (_, tag) = parse_tuple!(attr, Option<Token![as]>, Option<Ident>);
    let impl_block = parse2::<ItemImpl>(item.clone()).expect("parse impl block failed");
    let Type::Path(type_path) = impl_block.self_ty.as_ref().clone() else {
        panic!("unknown impl syntax")
    };
    let PathSegment {
        ident: self_type,
        arguments: generics,
    } = type_path.path.segments.last().unwrap();

    let trait_name = tag.unwrap_or(format_ident!("{self_type}Empl"));

    let where_clause = &impl_block.generics.where_clause;
    let sigs = impl_fn_signature(&impl_block);
    let defs = impl_fn_defination(&impl_block);

    let empl = quote! {
        pub trait #trait_name #generics
            #where_clause
        {
            #(#sigs;)*
        }

        impl #generics #trait_name #generics for #self_type #generics
            #where_clause
        {
            #(#defs)*
        }
    };
    empl
}

fn impl_fn_signature(impl_: &ItemImpl) -> impl Iterator<Item = &Signature> {
    impl_.items.iter().filter_map(|item| {
        let ImplItem::Fn(func) = item else { None? };
        Some(&func.sig)
    })
}

fn impl_fn_defination(impl_: &ItemImpl) -> impl Iterator<Item = ImplItemFn> {
    impl_.items.iter().filter_map(|item| {
        let ImplItem::Fn(func) = item else { None? };
        let mut func = func.clone();
        func.vis = syn::Visibility::Inherited;
        Some(func)
    })
}
