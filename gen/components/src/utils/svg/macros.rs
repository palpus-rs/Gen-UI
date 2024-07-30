#[macro_export]
macro_rules! impl_auto_value {
    ($($t:ty),*) => {
        $(
            impl AutoValue for $t {
                fn as_auto(v: Self) -> Auto<Self> {
                    Auto(Some(v))
                }
            }
        )*
    };
}
