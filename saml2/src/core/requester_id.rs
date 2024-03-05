use std::cell::Ref;

use crate::{common::SAML2Obj, error::SAMLError, xml::XmlObject};

#[derive(Debug, Default)]
pub struct RequesterID {
    value: Option<String>,
}

impl SAML2Obj for RequesterID {}

impl RequesterID {
    const ELEMENT_NAME: &'static str = "RequesterID";
    const NS_PREFIX: &'static str = "saml2";
    const NS_URI: &'static str = "urn:oasis:names:tc:SAML:2.0:assertion";

    #[inline]
    pub fn value(&self) -> Option<&str> {
        self.value.as_ref().map(|s| s.as_str())
    }

    #[inline]
    pub fn set_value(&mut self, value: Option<String>) {
        self.value = value;
    }
}

impl TryFrom<Ref<'_, XmlObject>> for RequesterID {
    type Error = SAMLError;

    fn try_from(object: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        match object.text() {
            Some(value) => Ok(RequesterID {
                value: Some(value.to_string()),
            }),
            None => Err(SAMLError::UnmarshallingError("Invalid XML".to_string())),
        }
    }
}

impl TryFrom<RequesterID> for XmlObject {
    type Error = SAMLError;

    fn try_from(requester_id: RequesterID) -> Result<Self, Self::Error> {
        let mut xml_object = XmlObject::new(
            Some(RequesterID::NS_PREFIX.to_string()),
            RequesterID::ELEMENT_NAME.to_string(),
            Some(RequesterID::NS_URI.to_string()),
        );
        xml_object.add_namespace(
            RequesterID::NS_PREFIX.to_string(),
            RequesterID::NS_URI.to_string(),
        );
        xml_object.set_text(requester_id.value);
        Ok(xml_object)
    }
}
