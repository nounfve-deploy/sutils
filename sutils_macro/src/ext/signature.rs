use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{FnArg, Ident, PatType, Receiver, ReturnType, Signature, Token, punctuated::Punctuated};

#[allow(unused)]
pub trait SignatureExt {
    fn parse_signature(&self) -> FnSigParsed;
    fn lambda_sig(&self) -> TokenStream;
    fn to_inner_fn(&self) -> Self;
}

impl SignatureExt for Signature {
    fn parse_signature(&self) -> FnSigParsed {
        let id = self.ident.clone();
        let mut this = None;
        let var_bind = self
            .inputs
            .iter()
            .filter_map(|arg| match arg {
                FnArg::Receiver(recv) => {
                    this = Some(recv.clone());
                    None
                }
                FnArg::Typed(pat) => Some(pat.clone()),
            })
            .collect::<Vec<_>>();
        let ret_ty = self.output.clone();
        FnSigParsed {
            id,
            this,
            var_bind,
            ret_ty,
        }
    }

    fn lambda_sig(&self) -> TokenStream {
        let FnSigParsed { this, var_bind, .. } = self.parse_signature();
        let this = this.map(|_| quote! { _self, });
        let input = var_bind
            .iter()
            .map(|arg| format!("{},", arg.to_token_stream()))
            .collect::<String>()
            .parse::<TokenStream>()
            .unwrap();
        quote! {
            |#this #input|
        }
    }

    fn to_inner_fn(&self) -> Self {
        let mut this = self.clone();
        this.ident = format_ident!("_inner_{}", this.ident.to_string());
        this
    }
}

#[derive(Clone)]
pub struct FnSigParsed {
    pub id: Ident,
    pub this: Option<Receiver>,
    pub var_bind: Vec<PatType>,
    pub ret_ty: ReturnType,
}

impl FnSigParsed {
    pub fn fn_sig(&self) -> TokenStream {
        let FnSigParsed {
            this,
            id,
            var_bind,
            ret_ty,
            ..
        } = self.clone();
        let this = this.map(|this| quote! { #this, });
        let input = var_bind
            .iter()
            .collect::<Punctuated<_, Token![,]>>();
        quote! {
            fn #id(#this #input) #ret_ty
        }
    }

    pub fn call_vars(&self) -> TokenStream {
        let input = self
            .var_bind
            .iter()
            .map(|PatType { pat, .. }| quote! { #pat })
            .collect::<Punctuated<_, Token![,]>>();
        quote! { #input }
    }
}
