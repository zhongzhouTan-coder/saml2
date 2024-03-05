use crate::common::SAML2Obj;

#[derive(Debug, Clone)]
pub struct StatusMessage {
    value: Option<String>,
}

impl SAML2Obj for StatusMessage {}
