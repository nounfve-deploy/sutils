#[super::PutInMacro(inline_macro)]
macro_rules! lazy_const {
    ($V:vis const $Ident:ident :$T_ignore:path = $T:path{
        $($Field:ident:$Val:expr,)*
    };) => {
        $V static $Ident: std::sync::LazyLock<$T> = std::sync::LazyLock::new(||{
            $T {
                $($Field:($Val).into(),)*
            }
        });
    };
}
