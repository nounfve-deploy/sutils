use syn::parse::{Parse, ParseStream, discouraged::Speculative};

use crate::sutils::thread_contest::ThreadContext;

#[allow(unused)]
pub trait ParseExt<T> {
    fn parse(stream: ParseStream) -> syn::Result<T>;
}

impl<T> ParseExt<Option<T>> for Option<T>
where
    T: Parse,
{
    fn parse(stream: ParseStream) -> syn::Result<Option<T>> {
        let fork = stream.fork();
        match T::parse(&fork) {
            Ok(val) => {
                stream.advance_to(&fork);
                Ok(Some(val))
            }
            Err(_) => Ok(None),
        }
    }
}

pub struct Tuple<T> {
    pub func: Box<dyn FnMut(ParseStream) -> syn::Result<T>>,
    pub result: Option<T>,
}

macro_rules! parse_tuple {
    ($stream:expr,$($T:ty),*) => {{
        use syn::parse::ParseStream;
        use syn::parse::Parse;
        use syn::parse2;
        use crate::ext::tuple_parse::Tuple;
        use crate::sutils::thread_contest::ThreadContext;

        let func = |stream: ParseStream| -> syn::Result<_> {
            let result = ($(
                <$T>::parse(stream)?,
            )*);
            Ok(result)
        };
        let func = Box::new(func);
        let result = None;
        ThreadContext::current().set(Tuple { func, result });
        parse2::<Tuple<($(
            $T,
        )*)>>($stream).unwrap().result.unwrap()
    }};
}

pub(crate) use parse_tuple;

impl<T: 'static> Parse for Tuple<T> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let Tuple { mut func, .. } = ThreadContext::current().get::<Self>().unwrap();
        let result = Some(func(input)?);
        Ok(Self { func, result })
    }
}
