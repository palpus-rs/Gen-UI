use std::fmt::Display;

use gen_utils::error::Errors;
use proc_macro2::TokenStream;
use syn::parse_str;

use crate::Function;

/// MakepadShader
/// 用于提供 Makepad 的着色器, 从BuiltinColor中获取并转化为着色器代码
#[derive(Debug, Clone)]
pub struct MakepadShader(pub TokenStream);

impl TryFrom<&Function> for MakepadShader {
    type Error = Errors;

    fn try_from(value: &Function) -> Result<Self, Self::Error> {
        if value.get_name() == "shader" {
            let tk = value
                .get_params()
                .clone()
                .expect("shader function must have params");
            let tk =
                parse_str::<TokenStream>(tk.get(0).expect("shader function must have one param"))
                    .unwrap();

            return Ok(Self(tk));
        }
        return Err(Errors::ParseError(format!(
            "{} can not convert to MakepadShader",
            value.get_name()
        )));
    }
}

impl Display for MakepadShader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

#[cfg(test)]
mod test_shader {
    use crate::target::parse_style;

    #[test]
    fn shader_quad() {
        let style = r#"
        .app{
            background_color : shader(|self|{
                fn pixel(self) -> vec4{
                    return #FFF
                }
            });
        }
        "#;
        let res = parse_style(style).unwrap();

        let node = res[0].clone();

        match node {
            crate::ASTNodes::Tag(_) => todo!(),
            crate::ASTNodes::Comment(_) => todo!(),
            crate::ASTNodes::Style(s) => {
                let style = *s;
                let values = style.get_props().unwrap().values().last().unwrap();

                let fn_v = values.is_fn_and_get().unwrap().clone();
                dbg!(fn_v);
            }
        }
    }
}
