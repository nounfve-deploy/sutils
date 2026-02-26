#![allow(non_snake_case)]

mod context_func;
mod into_lifetime;
mod macros;
mod singleton;

#[crate::macros::mod_use_all] 
struct UseAll;