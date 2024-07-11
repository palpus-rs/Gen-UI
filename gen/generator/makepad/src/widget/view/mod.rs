/// view
mod prop;
mod prop_ptr;
/// RectView and RectShadowView
mod rect;
/// RoundedView and RoundedShadowView
mod round;
/// ScrollXView and ScrollYView and ScrollXYView
mod scroll;
/// SolidView
mod solid;
mod r#trait;

pub use prop::ViewProps;
pub use prop_ptr::ViewPropPtr;
pub use r#trait::*;
pub use rect::*;
pub use round::*;
pub use scroll::*;
pub use solid::*;

#[macro_export]
macro_rules! inherits_view {
    ($t: ident) => {
        #[derive(Debug, Clone, Default)]
        pub struct $t(pub ViewProps);

        impl DynProps for $t {
            fn prop_bind(
                prop: &gen_parser::PropsKey,
                value: &gen_parser::Value,
                is_prop: bool,
                ident: &str,
            ) -> proc_macro2::TokenStream {
                ViewProps::prop_bind(prop, value, is_prop, ident)
            }
        }

        impl StaticProps for $t {
            fn props(props: &std::collections::HashMap<gen_parser::PropsKey, gen_parser::Value>) -> Self
            where
                Self: Sized,
            {
                Self(ViewProps::props(props))
            }

            fn prop(&mut self, prop_name: &str, value: &gen_parser::Value) -> () {
                self.0.prop(prop_name, value)
            }
        }

        impl Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        props_to_token!($t);
    };
}