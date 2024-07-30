use nom::IResult;

use super::{encode::EnCoding, parser::parse_properties};
use std::fmt::Display;
use std::fmt::{self, Formatter};

#[derive(Debug)]
pub struct Header {
    version: Option<String>,
    encoding: Option<EnCoding>,
}

impl Header {
    pub fn from_str(s: &str) -> IResult<&str, Header> {
        // parse `<?xml version="1.0" encoding="UTF-8"?>`
        let mut s = s.trim();
        let _ = s.starts_with("<?xml")
            || return Err(nom::Err::Error(nom::error::Error::new(
                s,
                nom::error::ErrorKind::Tag,
            )));
        s = s.trim_start_matches("<?xml");
        // parse version and encoding
        let (mut s, kvs) = parse_properties(s).unwrap();
        let mut version = None;
        let mut encoding = None;

        kvs.into_iter().for_each(|(k, v)| match k {
            "version" => {
                let _ = version.replace(v.to_string());
            }
            "encoding" => {
                let _ = encoding.replace(v.parse().unwrap());
            }
            _ => {}
        });

        s = s.trim();
        if !s.starts_with("?>") {
            return Err(nom::Err::Error(nom::error::Error::new(
                s,
                nom::error::ErrorKind::Tag,
            )));
        } else {
            s = s.trim_start_matches("?>");
        }

        Ok((s, Header { version, encoding }))
    }
}

impl Display for Header {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_fmt(format_args!(
            "<?xml version=\"{}\" encoding=\"{}\"?>",
            self.version.as_ref().unwrap_or(&"1.0".to_string()),
            self.encoding.as_ref().unwrap_or(&EnCoding::default())
        ))
    }
}
