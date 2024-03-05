use std::cell::Ref;

use crate::{common::SAML2Obj, error::SAMLError, xml::XmlObject};

#[derive(Debug, Default)]
pub struct Audience {
    value: String,
}

impl SAML2Obj for Audience {}

impl Audience {
    pub const ELEMENT_NAME: &'static str = "Audience";
    pub const NS_PREFIX: &'static str = "saml2";
    pub const NS_URI: &'static str = "urn:oasis:names:tc:SAML:2.0:assertion";

    #[inline]
    pub fn value(&self) -> &str {
        &self.value
    }

    #[inline]
    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }
}

impl TryFrom<Ref<'_, XmlObject>> for Audience {
    type Error = SAMLError;

    fn try_from(object: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        match object.text() {
            Some(value) => Ok(Audience {
                value: value.to_string(),
            }),
            None => Err(SAMLError::UnmarshallingError("Invalid XML".to_string())),
        }
    }
}

impl TryFrom<Audience> for XmlObject {
    type Error = SAMLError;

    fn try_from(audience: Audience) -> Result<Self, Self::Error> {
        let mut xml_object = XmlObject::new(
            Some(Audience::NS_PREFIX.to_string()),
            Audience::ELEMENT_NAME.to_string(),
            Some(Audience::NS_URI.to_string()),
        );
        xml_object.add_namespace(
            Audience::NS_PREFIX.to_string(),
            Audience::NS_URI.to_string(),
        );
        xml_object.set_text(Some(audience.value.to_string()));
        Ok(xml_object)
    }
}
