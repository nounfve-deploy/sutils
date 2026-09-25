pub use sutils_macro::mod_use_all;

#[deprecated = "*ONLY* works when inlined,"]
#[macro_export]
macro_rules! MOD_USE_ALL {
    () => {
        #[$crate::mod_use_all]
        struct UseAll;
    };
}

pub use sutils_macro::FnWrap;
pub use sutils_macro::PutInMacro;
pub use sutils_macro::TraitExport;
pub use sutils_macro::ExternImpl;

#[macro_export]
macro_rules! DEFINE {
    (pub $Var:ident= $($Body:tt)*) => {
        DEFINE!{impl
            #[doc(hidden)]
            #[macro_export]
            $Var= $($Body)*
        }
        #[doc(inline)]
        pub use $Var;
    };
    ($Vis:vis $Var:ident= $($Body:tt)*) => {
        DEFINE!{$Vis impl
            #[allow(unused_macros)]
            $Var= $($Body)*
        }
        #[allow(unused)]
        $Vis use $Var;
    };
    ($Vis:vis impl
        $(#[$Meta:meta])*
        $Var:ident= $($Body:tt)*
    )=>{
        $(#[$Meta])*
        $Vis macro_rules! $Var {
            ()=> { $($Body)*};
            (^$head:tt)=> { $head $($Body)*};
            (use $ident:ident)=> { $ident!($($Body)*)};
            (const)=>{concat!($($Body)*)};
        }
    };
}

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
re_export!(mod exten_impl);
