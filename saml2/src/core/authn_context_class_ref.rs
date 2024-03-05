use std::cell::Ref;

use crate::{common::SAML2Obj, error::SAMLError, xml::XmlObject};

#[derive(Debug, Default)]
pub struct AuthnContextClassRef {
    value: Option<String>,
}

impl SAML2Obj for AuthnContextClassRef {}

impl AuthnContextClassRef {
    const ELEMENT_NAME: &'static str = "AuthnContextClassRef";
    const NS_PREFIX: &'static str = "saml2";
    const NS_URI: &'static str = "urn:oasis:names:tc:SAML:2.0:assertion";

    #[inline]
    pub fn value(&self) -> Option<&String> {
        self.value.as_ref()
    }

    #[inline]
    pub fn set_value(&mut self, value: Option<String>) {
        self.value = value;
    }
}

impl TryFrom<Ref<'_, XmlObject>> for AuthnContextClassRef {
    type Error = SAMLError;

    fn try_from(object: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        match object.text() {
            Some(value) => Ok(AuthnContextClassRef {
                value: Some(value.to_string()),
            }),
            None => Err(SAMLError::UnmarshallingError("Invalid XML".to_string())),
        }
    }
}

impl TryFrom<AuthnContextClassRef> for XmlObject {
    type Error = SAMLError;

    fn try_from(authn_context_class_ref: AuthnContextClassRef) -> Result<Self, Self::Error> {
        let mut xml_object = XmlObject::new(
            Some(AuthnContextClassRef::NS_PREFIX.to_string()),
            AuthnContextClassRef::ELEMENT_NAME.to_string(),
            Some(AuthnContextClassRef::NS_URI.to_string()),
        );
        xml_object.add_namespace(
            AuthnContextClassRef::NS_PREFIX.to_string(),
            AuthnContextClassRef::NS_URI.to_string(),
        );
        if let Some(value) = authn_context_class_ref.value {
            xml_object.set_text(Some(value.to_string()));
        }
        Ok(xml_object)
    }
}
