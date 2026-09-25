#[deprecated(note = "use `ExternImpl` instead")]
#[sutils_macro::PutInMacro(inline_macro)]
macro_rules! extern_impl {
    (impl $T:ty {
        $(
            $Vis:vis fn $FnName:ident 
                $(<$( $G:ident $(: $B:path)? ),* >)? 
            ($($Args:tt)*) $(->$Return:ty)? {
                $($Body:tt)*
            }
        )*
    } $(as $Tag:ident)?
    ) => {$crate::external::paste!{
        pub trait [<$T $($Tag)? Impl>]{$(
            fn $FnName 
                $(<$( $G $(:$B)? ),* >)? 
            ($($Args)*) $(->$Return) ?;
        )*}

        impl  [<$T $($Tag)? Impl>] for $T{$(
            fn $FnName 
                $(<$( $G $(:$B)? ),* >)? 
            ($($Args)*) $(->$Return)? {
                $($Body)*
            }
        )*}
    }};
}
