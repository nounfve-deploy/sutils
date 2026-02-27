#![allow(non_snake_case)]

mod context_func;
mod into_lifetime;
mod macros;
mod singleton;

pub use context_func::*;
pub use into_lifetime::*;
pub use macros::*;
pub use singleton::*;

/// disabled since rust-analyzer not follow using statement in macro
/// bug
#[cfg(false)]
#[sutils_macro::mod_use_all]
struct UseAll; 
