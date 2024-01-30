use std::fmt::Display;

use crate::error::SAMLError;

#[derive(Clone, Default, Debug)]
pub struct SAMLVersion {
    major: i32,
    minor: i32,
}

impl SAMLVersion {
    pub fn new(major: i32, minor: i32) -> SAMLVersion {
        SAMLVersion { major, minor }
    }

    pub fn from_string(version: &str) -> Result<SAMLVersion, SAMLError> {
        match version {
            "2.0" => Ok(SAMLVersion::new(2, 0)),
            "1.1" => Ok(SAMLVersion::new(1, 1)),
            "1.0" => Ok(SAMLVersion::new(1, 0)),
            _ => Err(SAMLError::UnmarshallingError(
                "unsupported SAML version".to_string(),
            )),
        }
    }

    pub fn get_major_version(&self) -> i32 {
        self.major
    }

    pub fn get_minor_version(&self) -> i32 {
        self.minor
    }
}

impl Display for SAMLVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}
