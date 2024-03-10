#[macro_export]
macro_rules! str_to_string_try_from {
    ($Target:ty) => {
        impl TryFrom<&String> for $Target {
            type Error = Errors;

            fn try_from(value: &String) -> Result<Self, Self::Error> {
                value.as_str().try_into()
            }
        }
    };
}
