use sutils_macro::PutInMacro;

use crate::{IntoOption, inline_macro};

pub trait StrExt<'s> {
    fn or_env(self, key: &str) -> std::borrow::Cow<'s, str>;
    fn split_when<F: FnMut(char) -> bool>(self, f: F) -> SplitWhen<'s, F>;
}

impl<'s> StrExt<'s> for &'s str {
    fn or_env(self, key: &str) -> std::borrow::Cow<'s, str> {
        let Ok(val) = std::env::var(key) else {
            return std::borrow::Cow::Borrowed(self);
        };
        std::borrow::Cow::Owned(val)
    }

    fn split_when<F: FnMut(char) -> bool>(self, when: F) -> SplitWhen<'s, F> {
        SplitWhen { remain: self, when }
    }
}

#[PutInMacro(inline_macro)] 
macro_rules! env_or {
    ($S:ident) => {
        &$crate::extension::str::StrExt::or_env($S, stringify!($S))
    };
}

pub struct SplitWhen<'s, F> {
    remain: &'s str,
    when: F,
}

impl<'s, F> Iterator for SplitWhen<'s, F>
where
    F: FnMut(char) -> bool,
{
    type Item = &'s str;

    fn next(&mut self) -> Option<Self::Item> {
        let Self { remain, when } = self;
        for (idx, char) in remain.chars().enumerate() {
            if !when(char) {
                continue;
            }
            let (matched, rest) = remain.split_at(idx + 1);
            let _ = std::mem::replace(remain, rest);

            // skip empty
            if matched.is_empty() {
                return self.next();
            }
            return matched.Some();
        }
        None
    }
}