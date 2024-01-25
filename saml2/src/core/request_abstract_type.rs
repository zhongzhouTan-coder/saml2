use std::time::Instant;

use super::{extensions::Extensions, issuer::Issuer, saml_version::SAMLVersion};

pub struct RequestAbstractType {
    id: String,
    version: SAMLVersion,
    issue_instant: Instant,
    destination: Option<String>,
    consent: Option<String>,
    issuer: Option<Issuer>,
    signature: Option<String>,
    extensions: Option<Extensions>,
}
