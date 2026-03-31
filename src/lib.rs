#![allow(non_snake_case)]

mod chained_strcut;
mod context_func;
mod into_lifetime;
mod leak_box;
mod macros;
mod singleton;
mod unsafe_ref;

pub use chained_strcut::*;
pub use context_func::*;
pub use into_lifetime::*;
pub use leak_box::*;
pub use macros::*;
pub use singleton::*;
pub use unsafe_ref::*;

/// disabled since rust-analyzer not follow using statement in macro
/// bug
#[cfg(false)]
#[sutils_macro::mod_use_all]
struct UseAll;
