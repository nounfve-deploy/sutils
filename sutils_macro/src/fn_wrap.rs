use std::mem;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemFn, Visibility, parse2};

pub fn fn_wrap_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut inner = parse2::<ItemFn>(item).expect("parse inner function failed");
    
    let submod_ident = format_ident!("{}__submod__", inner.sig.ident);
    let out_vis = mem::replace(&mut inner.vis, Visibility::Inherited);
    let out_ident = mem::replace(&mut inner.sig.ident, format_ident!("__inner__"));
    
    let submod = quote! {
        #[allow(non_snake_case)]
        mod #submod_ident {
            use super::*;
            #attr ! {} 
            #inner
        }
        #out_vis use #submod_ident::__wrapper__ as #out_ident;
    };
    submod
}
