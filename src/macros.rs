pub use sutils_macro::mod_use_all;

#[deprecated="*ONLY* works when inlined,"]
#[macro_export]
macro_rules! MOD_USE_ALL {
    () => {
        #[$crate::mod_use_all]
        struct UseAll;
    };
}
