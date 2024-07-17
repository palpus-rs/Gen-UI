use std::{fmt::Display, str::FromStr};

use crate::error::Errors;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Version {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
    /// Compare the major version of the current version with the given version.
    /// - Returns 0 if the major version is the same
    /// - a positive number if the current version is greater
    /// - a negative number if the current version is less.
    pub fn match_major(&self, major: u32) -> i32 {
        self.major as i32 - major as i32
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.major, self.minor, self.patch).cmp(&(other.major, other.minor, other.patch))
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl FromStr for Version {
    type Err = Errors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_version_item(item: Option<Result<u32, Errors>>) -> Result<u32, Errors> {
            match item {
                Some(item) => item,
                None => Ok(0),
            }
        }

        let mut iter = s.split('.').map(|x| {
            x.parse::<u32>().map_err(|e| {
                Errors::ParseError(format!("can not parse {}: {} to version item", e, x))
            })
        });

        let major = parse_version_item(iter.next())?;
        let minor = parse_version_item(iter.next())?;
        let patch = parse_version_item(iter.next())?;

        Ok(Version::new(major, minor, patch))
    }
}

#[cfg(test)]
mod test_version {
    #[test]
    fn from() {
        let v = "1.2.3".parse::<crate::compiler::Version>().unwrap();
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 3);
    }
    #[test]
    fn from2() {
        let v = "1.2".parse::<crate::compiler::Version>().unwrap();
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 0);
    }
}
