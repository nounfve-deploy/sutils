use std::collections::HashSet;

use crate::ext::punctuated::PunctuatedExt;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, ItemStruct, Token, parse2, punctuated::Punctuated};

pub fn singleton_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ItemStruct { ident, .. } = parse2(item.clone()).unwrap();
    let attr = <Punctuated<Ident, Token![,]>>::parse2(attr)
        .expect("parse decor failed")
        .iter()
        .map(|ident| ident.to_string())
        .collect::<HashSet<_>>();

    let submod_ident = format_ident!("{ident}__singleton__");
    let impl_singleton = quote! {
        mod #submod_ident{
            use std::sync::Mutex;
            pub(super) static mut __ONE__ :*mut super::#ident = std::ptr::null_mut();
            pub(super) static mut __MX__:Mutex<()> = Mutex::new(());
        }

        impl sutils::Singleton for #ident {
            fn One<'r>() -> &'r mut Self {
                unsafe {
                    if #submod_ident :: __ONE__ == std::ptr::null_mut() {
                        <Self as sutils::Singleton>::Set(Self::default());
                    }
                    &mut * #submod_ident :: __ONE__
                }
            }

            fn OneSafe<'l>() -> sutils::SideLock<'l, Self> {
                unsafe {
                    #[allow(static_mut_refs)]
                    let guard = #submod_ident :: __MX__.lock().unwrap();
                    use sutils::SideGuard;
                    guard.side_guard(Self::One())
                }
            }

            fn Set(self){
                unsafe{
                    if #submod_ident :: __ONE__ != std::ptr::null_mut() {
                        drop(Box::from_raw(#submod_ident :: __ONE__ ));
                    }
                    let boxed = Box::new(self);
                    let ptr = Box::into_raw(boxed);
                    #submod_ident :: __ONE__ = ptr;
                }
            }
        }
    };

    let zeroed = if attr.contains("zeroed") {
        Some(quote! {
            impl Default for #ident {
                fn default() -> Self {
                    #[allow(invalid_value)]
                    unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
                }
            }
        })
    } else {
        None
    };

    quote! {
        #item
        #impl_singleton
        #zeroed
    }
}
