use crate::error::Errors;

/// uppercase the first title case of the string
///
/// if the first title case is ascii alphabetic it will back the uppercase String
/// else back the original String
pub fn uppercase_title(s: &str) -> Result<String, Errors> {
    if let Some(res) = s
        .char_indices()
        .next() //get the first char
        .and_then(|(i, c)| {
            if c.is_ascii_alphabetic() {
                // but first char back
                Some(c.to_uppercase().collect::<String>() + &s[i + 1..])
            } else {
                None
            }
        })
    {
        return Ok(res);
    }
    Err(Errors::UppercaseTitleFail)
}

/// consume original String to surround String
///
/// format: `surround_left_sign`xxx`surround_right_sign`
pub fn surround(s: String, l: &str, r: &str) -> String {
    format!("{}{}{}", l, s, r)
}

/// convert camel to snake case
/// 1. View => view
/// 2. ViewName => view_name
pub fn camel_to_snake(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut result = String::new();

    for (i, &c) in chars.iter().enumerate() {
        if c.is_uppercase() && i != 0 {
            result.push('_');
        }
        result.push(c.to_lowercase().next().unwrap());
    }

    result
}

pub fn snake_to_camel(s: &str) -> String {
    if s.contains("_"){
        s.split('_')
        .map(|part| {
            let mut c = part.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect()
    }else{
        uppercase_title(s).unwrap()
    }
}

/// remove the link from type expr
/// find :: and remove the following
/// String::from => String
/// Type::link => Type
pub fn remove_expr_link(expr: String) -> String {
    expr.contains("::")
        .then(|| expr.split("::").next().unwrap().to_string())
        .unwrap_or(expr)
}
