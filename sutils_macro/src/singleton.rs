use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemStruct, parse2};

pub fn singleton_macro(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ItemStruct { ident, .. } = parse2(item.clone()).unwrap();

    let static_one_ident = format_ident!("_{ident}_one_");
    let impl_singleton = quote! {
        static mut #static_one_ident :*mut #ident = std::ptr::null_mut();

        impl sutils::Singleton for #ident {
            fn One<'r>() -> &'r mut Self {
                unsafe {
                    if #static_one_ident == std::ptr::null_mut() {
                        let boxed = Box::new(Self::default());
                        let ptr = Box::into_raw(boxed);
                        #static_one_ident = ptr;
                    }
                    &mut * #static_one_ident
                }
            }
        }
    };

    quote! {
        #item
        #impl_singleton
    }
    .into()
}
