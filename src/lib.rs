#![allow(nonstandard_style)]

mod chained_strcut;
mod context_func;
mod into_lifetime;
mod into_variant;
mod leak_box;
mod macros;
mod singleton;
mod thread_contest;
mod unsafe_ref;
pub mod external;

pub use chained_strcut::*;
pub use context_func::*;
pub use into_lifetime::*;
pub use into_variant::*;
pub use leak_box::*;
pub use macros::*;
pub use singleton::*;
pub use thread_contest::*;
pub use unsafe_ref::*;

/// disabled since rust-analyzer not follow using statement in macro
/// bug
#[cfg(false)]
#[sutils_macro::mod_use_all]
struct UseAll;
