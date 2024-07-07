pub fn float_to_str(num: f32) -> String {
    if num.fract() == 0.0 {
        format!("{}.0", num)
    } else {
        format!("{}", num)
    }
}

#[macro_export]
macro_rules! from_u_number {
    ($from: ident) => {
        impl From<$from> for Value {
            fn from(value: $from) -> Self {
                Value::USize(value as usize)
            }
        }
    };
}
#[macro_export]
macro_rules! from_i_number {
    ($from: ident) => {
        impl From<$from> for Value {
            fn from(value: $from) -> Self {
                Value::ISize(value as isize)
            }
        }
    };
}
