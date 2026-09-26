use sutils_macro::PutInMacro;

use crate::{inline_macro, macros::DEFINE};

#[PutInMacro(inline_macro)]
macro_rules! tracing_env_or_info {
    () => {{
        let level = tracing_subscriber::filter::LevelFilter::INFO.into();
        let level = tracing_subscriber::EnvFilter::builder()
            .with_default_directive(level)
            .from_env_lossy();
        let subscriber = tracing_subscriber::FmtSubscriber::builder()
            .with_env_filter(level)
            .with_line_number(true)
            .finish();
        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");
        tracing::debug!("tracing init finish");
    }};
}

/// (R)eturn (I)nto res(P)onse
#[PutInMacro(inline_macro)]
macro_rules! RIP {
    ($(@$F:ident)? $($expr:expr),*)=>{{
        use axum::response::IntoResponse;
        return $($F)? (IntoResponse::into_response(( $( $expr ),* )));
    }};
}

DEFINE!(pub signal_await= async{
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    let terminate = async {
        use tokio::signal::unix::*;
        signal(SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
});

DEFINE! {pub health = || -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("timestamp_error")
        .as_secs();
    format!("Ok @[{time}]:{}-{}", file!(), line!())
}}

DEFINE! {pub not_found = |uri: axum::http::Uri| {
    (axum::http::StatusCode::NOT_FOUND, uri.to_string())
}}

DEFINE! {pub tokio_rt_singleton =
    use sutils::Singleton;
    use tokio::runtime::{Builder, Runtime};

    #[Singleton]
    pub struct TokioSingleton {
        rt: Runtime,
    }

    pub trait TokioSingletonImpls<T> {
        fn await_singleton(self) -> T;
    }

    impl<F, T> TokioSingletonImpls<T> for F
    where
        F: Future<Output = T>,
    {
        fn await_singleton(self) -> T {
            TokioSingleton::One().rt.block_on(self)
        }
    }

    impl Default for TokioSingleton {
        fn default() -> Self {
            let rt = Builder::new_multi_thread()
                .worker_threads(2)
                .enable_all()
                .build()
                .unwrap();
            Self { rt }
        }
    }
}
