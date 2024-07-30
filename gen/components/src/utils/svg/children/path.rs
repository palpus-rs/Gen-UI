use std::fmt::Display;

use nom::IResult;

use super::CommonKVs;

#[derive(Debug, Clone, Default)]
pub struct Path {
    common: CommonKVs,
    d: Option<String>,
}

impl Path {
    pub fn from_str(s: &str) -> IResult<&str, Path> {
        let (s, (common, kvs)) = CommonKVs::from_str(s.trim()).unwrap();

        if kvs.is_none() {
            Ok((s, Path { common, d: None }))
        } else {
            let d = kvs.as_ref().unwrap().get("d").map(|v| v.to_string());
            Ok((s, Path { common, d }))
        }
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "<path d=\"{}\" {} />",
            self.d.as_ref().unwrap_or(&"".to_string()),
            self.common
        ))
    }
}
