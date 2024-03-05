use std::cell::Ref;

use crate::{common::SAML2Obj, error::SAMLError, xml::XmlObject};

#[derive(Default, Debug, Clone)]
pub struct StatusCode {
    value: String,
    status_code: Option<Box<StatusCode>>,
}

impl SAML2Obj for StatusCode {}

/// implement getter and setter for status code
impl StatusCode {
    const ATTRIB_VALUE: &'static str = "Value";

    const ELEMENT_NAME: &'static str = "StatusCode";
    const NS_PREFIX: &'static str = "saml2p";
    const NS_URI: &'static str = "urn:oasis:names:tc:SAML:2.0:protocol";

    const SUCCESS: &'static str = "urn:oasis:names:tc:SAML:2.0:status:Success";
    const REQUESTER: &'static str = "urn:oasis:names:tc:SAML:2.0:status:Requester";
    const RESPONDER: &'static str = "urn:oasis:names:tc:SAML:2.0:status:Responder";
    const VERSION_MISMATCH: &'static str = "urn:oasis:names:tc:SAML:2.0:status:VersionMismatch";
    const AUTHN_FAILED: &'static str = "urn:oasis:names:tc:SAML:2.0:status:AuthnFailed";
    const INVALID_ATTR_NAME_OR_VALUE: &'static str =
        "urn:oasis:names:tc:SAML:2.0:status:InvalidAttrNameOrValue";
    const INVALID_NAME_ID_POLICY: &'static str =
        "urn:oasis:names:tc:SAML:2.0:status:InvalidNameIDPolicy";
    const NO_AUTHN_CONTEXT: &'static str = "urn:oasis:names:tc:SAML:2.0:status:NoAuthnContext";
    const NO_AVAILABLE_IDP: &'static str = "urn:oasis:names:tc:SAML:2.0:status:NoAvailableIDP";
    const NO_PASSIVE: &'static str = "urn:oasis:names:tc:SAML:2.0:status:NoPassive";
    const NO_SUPPORTED_IDP: &'static str = "urn:oasis:names:tc:SAML:2.0:status:NoSupportedIDP";
    const PARTIAL_LOGOUT: &'static str = "urn:oasis:names:tc:SAML:2.0:status:PartialLogout";
    const PROXY_COUNT_EXCEEDED: &'static str =
        "urn:oasis:names:tc:SAML:2.0:status:ProxyCountExceeded";
    const REQUEST_DENIED: &'static str = "urn:oasis:names:tc:SAML:2.0:status:RequestDenied";
    const REQUEST_UNSUPPORTED: &'static str =
        "urn:oasis:names:tc:SAML:2.0:status:RequestUnsupported";
    const REQUEST_VERSION_DEPRECATED: &'static str =
        "urn:oasis:names:tc:SAML:2.0:status:RequestVersionDeprecated";
    const REQUEST_VERSION_TOO_HIGH: &'static str =
        "urn:oasis:names:tc:SAML:2.0:status:RequestVersionTooHigh";
    const REQUEST_VERSION_TOO_LOW: &'static str =
        "urn:oasis:names:tc:SAML:2.0:status:RequestVersionTooLow";
    const RESOURCE_NOT_RECOGNIZED: &'static str =
        "urn:oasis:names:tc:SAML:2.0:status:ResourceNotRecognized";
    const TOO_MANY_RESPONSES: &'static str = "urn:oasis:names:tc:SAML:2.0:status:TooManyResponses";
    const UNKNOWN_ATTR_PROFILE: &'static str =
        "urn:oasis:names:tc:SAML:2.0:status:UnknownAttrProfile";
    const UNKNOWN_PRINCIPAL: &'static str = "urn:oasis:names:tc:SAML:2.0:status:UnknownPrincipal";
    const UNSUPPORTED_BINDING: &'static str =
        "urn:oasis:names:tc:SAML:2.0:status:UnsupportedBinding";

    pub fn value(&self) -> &String {
        &self.value
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }

    pub fn status_code(&self) -> Option<&Box<StatusCode>> {
        self.status_code.as_ref()
    }

    pub fn set_status_code(&mut self, status_code: Option<Box<StatusCode>>) {
        self.status_code = status_code;
    }
}

impl TryFrom<Ref<'_, XmlObject>> for StatusCode {
    type Error = SAMLError;

    fn try_from(object: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        let mut status_code = StatusCode::default();
        for attribute in object.attributes() {
            match attribute.0.as_str() {
                StatusCode::ATTRIB_VALUE => {
                    status_code.set_value(attribute.1.to_string());
                }
                _ => {}
            }
        }
        Ok(status_code)
    }
}

impl TryFrom<StatusCode> for XmlObject {
    type Error = SAMLError;

    fn try_from(value: StatusCode) -> Result<Self, Self::Error> {
        let mut object = XmlObject::new(
            Some(StatusCode::NS_URI.to_string()),
            StatusCode::ELEMENT_NAME.to_string(),
            Some(StatusCode::NS_PREFIX.to_string()),
        );
        object.add_attribute(
            StatusCode::ATTRIB_VALUE.to_string(),
            value.value().to_string(),
        );
        Ok(object)
    }
}
