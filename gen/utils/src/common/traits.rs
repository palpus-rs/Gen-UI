/// impl for FixedString trait ----------------------------------------------------------------
#[macro_export]
macro_rules! split_fixed_impl {
    ($Str: ty) => {
        impl FixedString for $Str {
            fn split_fixed(&self, pat: &str) -> Vec<String> {
                split_fixed(self, pat)
            }
            
            fn split_fixed_option(&self, pat: &str) -> Option<Vec<String>> {
                let res = self.split_fixed(pat);
                if res.is_empty() {
                    None
                } else {
                    Some(res)
                }
            }
            fn is_inner_string(&self) -> bool {
                self.trim().starts_with('"') && self.ends_with('"')
            }
        }
    }
}
// -----------------------------------------------------------------------------------------------

// -----------------------------------------------------------------------------------------------
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