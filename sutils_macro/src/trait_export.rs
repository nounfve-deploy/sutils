use crate::ext::signature::SigExt;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{ImplItemFn, ItemImpl, parse2};

pub fn trait_export(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut impl_block = parse2::<ItemImpl>(item).expect("parse impl block failed");

    let ItemImpl {
        self_ty,
        trait_,
        items,
        attrs,
        ..
    } = &mut impl_block;
    let (_, trait_ty, _) = trait_.as_ref().unwrap();

    let drop_outer_attr = std::mem::replace(attrs, vec![]);
    let drop_outer_attr = drop_outer_attr
        .into_iter()
        .filter_map(|a| match a.style {
            syn::AttrStyle::Outer => Some(a.to_token_stream()),
            syn::AttrStyle::Inner(_) => {
                attrs.push(a);
                None
            }
        })
        .collect::<TokenStream>();
    let funcs = items
        .into_iter() //
        .filter_map(|item| match item {
            syn::ImplItem::Fn(impl_item_fn) => Some(impl_item_fn),
            _ => None,
        })
        .map(|ImplItemFn { sig, .. }| {
            let mut parse = sig.parse();
            (parse.this.take(), parse)
        })
        .map(|(this, parse)| {
            let fn_sig = parse.fn_sig();
            let ident = &parse.id;
            let self_resolver = this.map(|_| quote! {let _self = #attr;});
            let self_arg = if self_resolver.is_some() {
                Some(quote! { _self, })
            } else {
                None
            };
            let input = parse.call_vars();
            quote! {
                #drop_outer_attr
                pub #fn_sig {
                    #self_resolver
                    <#self_ty as #trait_ty>::#ident(#self_arg #input)
                }
            }
        })
        .collect::<TokenStream>();
    quote! {
        #impl_block
        #funcs
    }
}
