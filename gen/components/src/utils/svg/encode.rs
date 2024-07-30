use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnCoding{
    UTF8,
    UTF16,
    UTF32,
    ASCII,
    ISO8859,
    ISO2022,
    EUCJP,
    SJIS,
    GB2312,
    BIG5,
    KOI8R,
    KOI8U,
    UNKNOWN(String),
}

impl Default for EnCoding{
    fn default() -> Self {
        EnCoding::UTF8
    }
}

impl FromStr for EnCoding{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s{
            "UTF-8" => Ok(EnCoding::UTF8),
            "UTF-16" => Ok(EnCoding::UTF16),
            "UTF-32" => Ok(EnCoding::UTF32),
            "ASCII" => Ok(EnCoding::ASCII),
            "ISO-8859" => Ok(EnCoding::ISO8859),
            "ISO-2022" => Ok(EnCoding::ISO2022),
            "EUC-JP" => Ok(EnCoding::EUCJP),
            "SJIS" => Ok(EnCoding::SJIS),
            "GB2312" => Ok(EnCoding::GB2312),
            "BIG5" => Ok(EnCoding::BIG5),
            "KOI8-R" => Ok(EnCoding::KOI8R),
            "KOI8-U" => Ok(EnCoding::KOI8U),
            _ => Ok(EnCoding::UNKNOWN(s.to_string())),
        }
    }
}

impl Display for EnCoding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            EnCoding::UTF8 => f.write_str("UTF-8"),
            EnCoding::UTF16 => f.write_str("UTF-16"),
            EnCoding::UTF32 => f.write_str("UTF-32"),
            EnCoding::ASCII => f.write_str("ASCII"),
            EnCoding::ISO8859 => f.write_str("ISO-8859"),
            EnCoding::ISO2022 => f.write_str("ISO-2022"),
            EnCoding::EUCJP => f.write_str("EUC-JP"),
            EnCoding::SJIS => f.write_str("SJIS"),
            EnCoding::GB2312 => f.write_str("GB2312"),
            EnCoding::BIG5 => f.write_str("BIG5"),
            EnCoding::KOI8R => f.write_str("KOI8-R"),
            EnCoding::KOI8U => f.write_str("KOI8-U"),
            EnCoding::UNKNOWN(s) => f.write_str(s),
        }
    }
}