use std::cell::Ref;

use crate::{common::SAML2Obj, error::SAMLError, xml::XmlObject};

#[derive(Debug, Default)]
pub struct AuthenticatingAuthority {
    value: String,
}

impl SAML2Obj for AuthenticatingAuthority {}

impl AuthenticatingAuthority {
    const ELEMENT_NAME: &'static str = "AuthenticatingAuthority";

    const NS_PREFIX: &'static str = "saml2";
    const NS_URI: &'static str = "urn:oasis:names:tc:SAML:2.0:assertion";

    #[inline]
    pub fn value(&self) -> &str {
        &self.value
    }

    #[inline]
    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }
}

impl TryFrom<Ref<'_, XmlObject>> for AuthenticatingAuthority {
    type Error = SAMLError;

    fn try_from(object: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        match object.text() {
            Some(value) => Ok(AuthenticatingAuthority {
                value: value.to_string(),
            }),
            None => Err(SAMLError::UnmarshallingError("Invalid XML".to_string())),
        }
    }
}

impl TryFrom<AuthenticatingAuthority> for XmlObject {
    type Error = SAMLError;

    fn try_from(authenticating_authority: AuthenticatingAuthority) -> Result<Self, Self::Error> {
        let mut xml_object = XmlObject::new(
            Some(AuthenticatingAuthority::NS_PREFIX.to_string()),
            AuthenticatingAuthority::ELEMENT_NAME.to_string(),
            Some(AuthenticatingAuthority::NS_URI.to_string()),
        );
        xml_object.add_namespace(
            AuthenticatingAuthority::NS_PREFIX.to_string(),
            AuthenticatingAuthority::NS_URI.to_string(),
        );
        xml_object.set_text(Some(authenticating_authority.value.to_string()));
        Ok(xml_object)
    }
}
