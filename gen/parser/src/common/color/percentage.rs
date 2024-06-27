use std::str::FromStr;

use gen_utils::error::Errors;

/// 百分比
/// 语法: `percentage(%)`
#[derive(Debug, Clone, Copy)]
pub struct Percentage(pub f32);

impl Percentage {
    /// 修正百分比
    pub fn fix(&mut self, start: f32, end: f32, index: usize, len: usize) -> () {
        let step = (end - start) / (len as f32);
        self.0 = step * (index as f32);
    }
}

impl FromStr for Percentage {
    type Err = Errors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 匹配百分比语法，输入类似: 11.5%
        if s.ends_with("%") {
            let s = s.trim_end_matches("%");
            let p = s
                .parse::<f32>()
                .map_err(|_| Errors::ParseError(format!("parse percentage error: {}", s)))?;
            Ok(Percentage(p))
        } else {
            Err(Errors::ParseError(
                "parse percentage error, percentage need `%` as end".to_string(),
            ))
        }
    }
}
