//! mode:
//! - variable: let variable_name: variable_type = variable_value;
//! - funcation: let function_name: function_type = ||{ function_handle };

use nom::{branch::alt, bytes::complete::tag, IResult};

use crate::{ast::script::VariableType, common::trim, CONST, LET};

#[allow(dead_code)]
fn parse_var_type(input: &str) -> IResult<&str, VariableType> {
    match alt((trim(tag(LET)), trim(tag(CONST))))(input) {
        Ok((input, ty)) => {
            let ty = VariableType::from(ty);
            Ok((input, ty))
        }
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod test_script_parse {
    use crate::ast::script::VariableType;

    use super::parse_var_type;

    #[test]
    fn test_parse_var_type() {
        let input = r#"let a: &str = "hello""#;
        let input2 = r#"const B: &str = "world""#;

        let (remain1, ty1) = parse_var_type(input).unwrap();
        let (remain2, ty2) = parse_var_type(input2).unwrap();
        assert_eq!(remain1, r#"a: &str = "hello""#);
        assert_eq!(remain2, r#"B: &str = "world""#);
        assert_eq!(ty1, VariableType::Let);
        assert_eq!(ty2, VariableType::Const);
    }
}
