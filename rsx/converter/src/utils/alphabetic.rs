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
    let mut chars: Vec<char> = s.chars().collect();
    let mut result = String::new();

    for (i, &c) in chars.iter().enumerate() {
        if c.is_uppercase() && i != 0 {
            result.push('_');
        }
        result.push(c.to_lowercase().next().unwrap());
    }

    result
}
