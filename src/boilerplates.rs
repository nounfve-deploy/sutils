use sutils_macro::PutInMacro;

use crate::{DEFINE, inline_macro};

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
        tracing::info!("tracing init finish");
    }};
}

/// (R)eturn (I)nto res(P)onse
#[PutInMacro(inline_macro)]
macro_rules! RIP {
    ($($expr:expr),*) => {
        {
            use axum::response::IntoResponse;
            return IntoResponse::into_response(( $( ($expr) ),* ));
        }
    };
}

DEFINE! {pub health= || -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("timestamp_error")
        .as_secs();
    format!("Ok @[{time}]:{}-{}", file!(), line!())
}}

DEFINE! {pub not_found=|uri: axum::http::Uri| {
    (axum::http::StatusCode::NOT_FOUND, uri.to_string())
}}
