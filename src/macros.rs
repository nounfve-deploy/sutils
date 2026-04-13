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

#[macro_export]
macro_rules! DEFINE {
    ($Var:ident= $($Body:tt)*) => {
        DEFINE!{impl
            #[allow(unused_macros)]
            $Var= $($Body)*
        }
        #[allow(unused)]
        use $Var;
    };
    (pub $Var:ident= $($Body:tt)*) => {
        DEFINE!{impl
            #[doc(hidden)]
            #[macro_export]
            $Var= $($Body)*
        }
        pub use $Var;
    };
    (impl
        $(#[$Meta:meta])*
        $Var:ident= $($Body:tt)*
    )=>{
        $(#[$Meta])*
        macro_rules! $Var {
            ()=> { $($Body)*};
            (^$head:tt)=> { $head $($Body)*};
            (use $ident:ident)=> { $ident!($($Body)*)};
            (const)=>{concat!($($Body)*)};
        }
    };
}
