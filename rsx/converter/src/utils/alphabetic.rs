use crate::error::Errors;

/// uppercase the first title case of the string
/// 
/// if the first title case is ascii alphabetic it will back the uppercase String
/// else back the original String
pub fn uppercase_title(s:&str) -> Result<String,Errors>{
    if let Some(res) = s.char_indices()
        .next() //get the first char
        .and_then(|(i,c)|{
            if c.is_ascii_alphabetic(){
                // but first char back
                Some(c.to_uppercase().collect::<String>()+&s[i+1..])
            }else{
                None
            }
        }) {
        return Ok(res);
    }
    Err(Errors::UppercaseTitleFail)
}


/// consume original String to surround String
/// 
/// format: `surround_left_sign`xxx`surround_right_sign`
pub fn surround(s:String,l:&str,r:&str) -> String{
    format!("{}{}{}",l,s,r)
}
