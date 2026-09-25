
#[sutils_macro::PutInMacro(inline_macro)]
macro_rules! command {
    (async $($Any:tt)*)=>{
        $crate::macros::command_inner!(use tokio::process::Command, $($Any)*)
    };
    ($($Any:tt)*)=>{
        $crate::macros::command_inner!(use std::process::Command, $($Any)*)
    };
}

#[sutils_macro::PutInMacro(inline_macro)]
macro_rules! command_inner {
    (use $Cmd:ty, $Bin:expr) => {
        $crate::macros::command_inner!(impl $Cmd, $Bin,)
    };
    (use $Cmd:ty,$Bin:expr => $($Arg:expr),*) => {
        $crate::macros::command_inner!(impl $Cmd, $Bin,.args([$($Arg,)*]))
    };
    (impl $Cmd:ty, $Bin:expr, $($Trail:tt)*)=>{
        <$Cmd>::new($Bin)
            .env("RUST_LOG", "error")
            .stdin(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            $($Trail)*
    };
}
