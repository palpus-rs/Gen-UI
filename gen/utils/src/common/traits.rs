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