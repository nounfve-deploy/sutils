#![allow(non_snake_case)]

mod context_func;
mod into_lifetime;
mod singleton;
mod macros;

#[crate::macros::mod_use_all]
struct UseAll;
