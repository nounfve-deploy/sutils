use std::{collections::HashSet, env, path::PathBuf};

use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{File, Item, parse2};

pub fn mod_use_all_macro(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("Failed to get CARGO_MANIFEST_DIR");
    let file = proc_macro::Span::call_site().file();
    let mut path = PathBuf::new();
    path.push(manifest_dir);
    path.push(file);

    let body = std::fs::read_to_string(&path)
        .expect(&format!("read file error {path:?}"))
        .parse::<TokenStream>()
        .expect("parse token stream failed");
    let File { items, .. } = parse2(body).expect("parse failed");

    macro_rules! filter_item {
        ($($if:tt)*) => {
            items
                .iter()
                .filter_map(|i| match i {
                    $($if)*,
                    _ => None,
                })
                .collect::<HashSet<_>>()
        };
    }

    let mut mod_list = filter_item!(Item::Mod(m)=>Some(m.ident.clone()));
    let use_list = filter_item!(Item::Use(u)=>Some(u.tree.to_token_stream().to_string()));

    // remove used mod
    use_list
        .iter()
        .map(|i| i.split(" :: ").next().unwrap())
        .for_each(|e| {
            mod_list.remove(&format_ident!("{e}"));
        });

    let use_statements = mod_list
        .into_iter()
        .map(|module| {
            quote! {
                pub use #module::*;
            }
        })
        .collect::<TokenStream>();

    // println!("{:?}", use_statements.to_string());
    use_statements
}
