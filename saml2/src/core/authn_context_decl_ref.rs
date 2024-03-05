use std::cell::Ref;

use crate::{common::SAML2Obj, error::SAMLError, xml::XmlObject};

#[derive(Debug, Default)]
pub struct AuthnContextDeclRef {
    value: Option<String>,
}

impl SAML2Obj for AuthnContextDeclRef {}

impl AuthnContextDeclRef {
    const ELEMENT_NAME: &'static str = "AuthnContextDeclRef";
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

impl TryFrom<Ref<'_, XmlObject>> for AuthnContextDeclRef {
    type Error = SAMLError;

    fn try_from(object: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        match object.text() {
            Some(value) => Ok(AuthnContextDeclRef {
                value: Some(value.to_string()),
            }),
            None => Err(SAMLError::UnmarshallingError("Invalid XML".to_string())),
        }
    }
}

impl TryFrom<AuthnContextDeclRef> for XmlObject {
    type Error = SAMLError;

    fn try_from(authn_context_decl_ref: AuthnContextDeclRef) -> Result<Self, Self::Error> {
        let mut xml_object = XmlObject::new(
            Some(AuthnContextDeclRef::NS_PREFIX.to_string()),
            AuthnContextDeclRef::ELEMENT_NAME.to_string(),
            Some(AuthnContextDeclRef::NS_URI.to_string()),
        );
        xml_object.add_namespace(
            AuthnContextDeclRef::NS_PREFIX.to_string(),
            AuthnContextDeclRef::NS_URI.to_string(),
        );
        if let Some(value) = authn_context_decl_ref.value {
            xml_object.set_text(Some(value.to_string()));
        }
        Ok(xml_object)
    }
}
