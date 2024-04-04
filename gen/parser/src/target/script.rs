//! mode:
//! - variable: let | const variable_name: variable_type = variable_value;
//! - funcation: let function_name: function_type = ||{ function_handle };

use proc_macro2::TokenStream;

#[allow(dead_code)]
pub fn parse_script(input: &str) -> Result<TokenStream, crate::error::Error> {
    let input = format!("{{ {} }}", input);
    // make input to TokenStream
    let token = match input.parse::<TokenStream>() {
        Ok(t) => return Ok(t),
        Err(_) => {
            return Err(crate::error::Error::parse_error(
                "cannot parse rsx script to rust TokenStream!",
            ));
        }
    };

    // // token to ast
    // match parse2::<Block>(token) {
    //     Ok(ast) => Ok(ast),
    //     Err(_) => Err(crate::error::Error::parse_error(
    //         "cannot convert TokenStream to rust Block!",
    //     )),
    // }
}

#[cfg(test)]
mod test_script_parse {
    use proc_macro2::TokenStream;
    use syn::{parse2, parse_str, Block, Expr, Stmt};

    #[test]
    fn test_syn_parse_var() {
        let rsx_code_var = r#"let counter: usize = 0_usize;"#;

        let ast_var = parse_str::<Stmt>(rsx_code_var).unwrap();
        dbg!(ast_var);
    }

    #[test]
    fn test_syn_parse_fn() {
        let rsx_code_fn = r#"
        let mut btn_click = ||{
            log!("BUTTON CLICKED {}", counter);
            counter += 1;
          }
        "#;

        let ast_fn = parse_str::<Expr>(rsx_code_fn).unwrap();
        dbg!(ast_fn);
    }

    #[test]
    fn test_parse_mixin() {
        let code = r#"
        let mut counter:usize = 0_usize;

        let mut click = ||{
            counter += 1;
        };
        "#;
        // to tokenStream
        let token: TokenStream = format!("{{ {} }}", code).parse().expect("error token");

        // to ast -> Block
        let ast = parse2::<Block>(token).expect("ast  error");
        dbg!(ast);
    }
}
