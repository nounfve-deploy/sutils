use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemStruct, parse2};

pub fn singleton_macro(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ItemStruct { ident, .. } = parse2(item.clone()).unwrap();

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
                        let boxed = Box::new(Self::default());
                        let ptr = Box::into_raw(boxed);
                        #submod_ident :: __ONE__ = ptr;
                    }
                    &mut * #submod_ident :: __ONE__
                }
            }

            fn OneSafe<'l>() -> sutils::SideGuardStruct<'l, Self> {
                unsafe {
                    #[allow(static_mut_refs)]
                    let guard = #submod_ident :: __MX__.lock().unwrap();
                    use sutils::SideGuard;
                    guard.side_guard(Self::One())
                }
            }
        }
    };

    quote! {
        #item
        #impl_singleton
    }
}
