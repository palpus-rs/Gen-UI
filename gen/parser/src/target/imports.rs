use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::char,
    multi::many0,
    sequence::delimited,
    IResult,
};
use proc_macro2::TokenStream;
use syn::{parse2, parse_str, Stmt};

use crate::common::trim;

/// Parse braces: `import!{}`
/// return `(remain, content)`
fn parse_braces(input: &str) -> IResult<&str, &str> {
    delimited(
        delimited(trim(tag("import")), char('!'), trim(char('{'))),
        take_while(|c| c != '}'),
        char('}'),
    )(input)
}

/// Parse parentheses: `import!();`
fn parse_parentheses(input: &str) -> IResult<&str, &str> {
    delimited(
        delimited(trim(tag("import")), char('!'), trim(char('('))),
        take_while(|c| c != ')'),
        tag(");"),
    )(input)
}

/// Parse imports from a file
///
/// find `import!{}`|`import!();` macro
pub fn parse_imports(input: &str) -> IResult<&str, Vec<&str>> {
    trim(many0(alt((parse_braces, parse_parentheses))))(input)
}

pub fn parse_imports_to_token(input: &str) -> Option<TokenStream> {
    // let a = parse_imports(input).unwrap();
    let import = parse_imports(input).unwrap().1.join(" ");

    if import.is_empty() {
        return None;
    }

    let import = format!("import!{{ {} }}", import.trim());
    let import: Stmt =
        parse2(parse_str(&import).expect("parse import failed, please check your import! macro"))
            .unwrap();

    if let Stmt::Macro(mac) = import {
        return Some(mac.mac.tokens);
    }
    None
}

#[cfg(test)]
mod test_input {
    #[test]
    fn test_import_macro() {
        let input1 = r#"import ! { crate::a::b; crate::c::d; crate::m::n::l; }"#;
        let input2 = "import!(crate::p::e;);";
        let res = super::parse_braces(&input1);
        let res2 = super::parse_parentheses(input2);
        assert!(res.is_ok());
        assert!(res2.is_ok());
    }
    #[test]
    fn test_parse_imports() {
        let input1 = r#"import!{ crate::a::b; crate::c::d; crate::m::n::l; }"#;
        let input2 = "import!(crate::p::e;);";
        let input3 = "use create::p::e;";
        let content1 = crate::target::parse_imports(input1).unwrap();
        let content2 = crate::target::parse_imports(input2).unwrap();
        let content3 = crate::target::parse_imports(input3).unwrap();
        assert_eq!(
            content1.1.join(" ").trim(),
            "crate::a::b; crate::c::d; crate::m::n::l;"
        );
        assert_eq!(content2.1.join(" ").trim(), "crate::p::e;");
        assert_eq!(content3.1.join(" ").trim(), "");
        // dbg!(super::parse_imports_token(&input1));
    }
    #[test]
    fn test_parse_imports_tk() {
        let input1 = r#"import ! { crate::a::b; crate::c::d; crate::m::n::l; }"#;
        let input2 = "import! (crate::p::e;);";
        let input3 = "use create::p::e;";
        let content1 = crate::target::parse_imports_to_token(input1).unwrap();
        let content2 = crate::target::parse_imports_to_token(input2).unwrap();
        let content3 = crate::target::parse_imports_to_token(input3);
        dbg!(content1.to_string());
        dbg!(content2.to_string());
        dbg!(content3);
    }
}
