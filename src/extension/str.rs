use sutils_macro::PutInMacro;

use crate::inline_macro;

pub trait StrExt<'s> {
    fn or_env(self, key: &str) -> std::borrow::Cow<'s, str>;
}

impl<'s> StrExt<'s> for &'s str {
    fn or_env(self, key: &str) -> std::borrow::Cow<'s, str> {
        let Ok(val) = std::env::var(key) else {
            return std::borrow::Cow::Borrowed(self);
        };
        std::borrow::Cow::Owned(val)
    }
}

#[PutInMacro(inline_macro)]
macro_rules! env_or {
    ($S:ident) => {
        & $crate::extension::str::StrExt::or_env($S, stringify!($S))
    };
}
