#[super::PutInMacro(inline_macro)]
macro_rules! DEFINE {
    (pub $Var:ident = $($Body:tt)*) => {
        DEFINE!{impl
            #[doc(hidden)]
            #[macro_export]
            $Var= $($Body)*
        }
        #[doc(inline)]
        pub use $Var;
    };
    ($Vis:vis $Var:ident = $($Body:tt)*) => {
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
