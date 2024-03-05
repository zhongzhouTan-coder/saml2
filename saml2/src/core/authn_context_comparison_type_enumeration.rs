use std::str::FromStr;

use crate::{common::SAML2Obj, error::SAMLError};

#[derive(Clone, Debug)]
pub enum AuthnContextComparisonTypeEnumeration {
    EXACT(String),
    MINIMUM(String),
    MAXIMUM(String),
    BETTER(String),
}

impl SAML2Obj for AuthnContextComparisonTypeEnumeration {}

impl FromStr for AuthnContextComparisonTypeEnumeration {
    type Err = SAMLError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "exact" => Ok(AuthnContextComparisonTypeEnumeration::EXACT(s.to_string())),
            "minimum" => Ok(AuthnContextComparisonTypeEnumeration::MINIMUM(
                s.to_string(),
            )),
            "maximum" => Ok(AuthnContextComparisonTypeEnumeration::MAXIMUM(
                s.to_string(),
            )),
            "better" => Ok(AuthnContextComparisonTypeEnumeration::BETTER(s.to_string())),
            _ => Err(SAMLError::UnmarshallingError(format!(
                "Invalid AuthnContextComparisonTypeEnumeration: {}",
                s
            ))),
        }
    }
}

impl ToString for AuthnContextComparisonTypeEnumeration {
    fn to_string(&self) -> String {
        match self {
            AuthnContextComparisonTypeEnumeration::EXACT(s) => s.to_string(),
            AuthnContextComparisonTypeEnumeration::MINIMUM(s) => s.to_string(),
            AuthnContextComparisonTypeEnumeration::MAXIMUM(s) => s.to_string(),
            AuthnContextComparisonTypeEnumeration::BETTER(s) => s.to_string(),
        }
    }
}
