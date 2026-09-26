pub use sutils_macro::mod_use_all;

#[deprecated = "*ONLY* works when inlined,"]
#[macro_export]
macro_rules! MOD_USE_ALL {
    () => {
        #[$crate::mod_use_all]
        struct UseAll;
    };
}

pub use sutils_macro::ExternImpl;
pub use sutils_macro::FnWrap;
pub use sutils_macro::PutInMacro;
pub use sutils_macro::TraitExport;

#[macro_export]
macro_rules! inline_macro {
    (
        $(#[$Meta:meta])*
        macro_rules! $M:ident $($B:tt)*
    ) => {
        #[macro_export]
        #[doc(hidden)]
        $(#[$Meta])*
        macro_rules! $M $($B)*

        #[doc(inline)]
        #[allow(unused)]
        pub use $M;
    };
}

#[sutils_macro::PutInMacro(inline_macro)]
macro_rules! re_export {
    (mod $M:ident) => {
        mod $M;
        pub use $M::*;
    };
}

re_export!(mod command);
re_export!(mod define);
