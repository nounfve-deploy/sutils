use proc_macro2::TokenStream;
use syn::{
    Error,
    parse::{Parse, Parser},
    punctuated::Punctuated,
};

pub trait PunctuatedExt<T>: Sized {
    fn parse2(token: TokenStream) -> Result<Self, Error>;
    fn pop_first(self) -> (Option<T>, Option<Self>);
}

impl<T, P> PunctuatedExt<T> for Punctuated<T, P>
where
    T: Parse,
    P: Parse + Default,
{
    fn parse2(tokens: TokenStream) -> Result<Self, Error> {
        Self::parse_terminated.parse2(tokens)
    }

    fn pop_first(self) -> (Option<T>, Option<Self>) {
        let mut iter = self.into_iter();
        let first = iter.next();
        let mut this = Some(iter.collect::<Self>());
        
        if this.as_ref().unwrap().is_empty() {
            this.take();
        };

        (first, this)
    }
}
