use std::cell::Ref;

use crate::{common::SAML2Obj, error::SAMLError, xml::XmlObject};

#[derive(Default, Debug)]
pub struct CarriedKeyName {
    value: String,
}

impl SAML2Obj for CarriedKeyName {}

impl CarriedKeyName {
    const ELEMENT_NAME: &'static str = "CarriedKeyName";

    const NS_PREFIX: &'static str = "xenc";
    const NS_URI: &'static str = "http://www.w3.org/2001/04/xmlenc#";

    #[inline]
    pub fn value(&self) -> &str {
        &self.value
    }

    #[inline]
    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }
}

impl TryFrom<Ref<'_, XmlObject>> for CarriedKeyName {
    type Error = SAMLError;

    fn try_from(xml_object: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        match xml_object.text() {
            Some(value) => Ok(CarriedKeyName {
                value: value.to_string(),
            }),
            None => Err(SAMLError::UnmarshallingError("Invalid XML".to_string())),
        }
    }
}

impl TryFrom<CarriedKeyName> for XmlObject {
    type Error = SAMLError;

    fn try_from(carried_key_name: CarriedKeyName) -> Result<Self, Self::Error> {
        let mut xml_object = XmlObject::new(
            Some(CarriedKeyName::NS_PREFIX.to_string()),
            CarriedKeyName::ELEMENT_NAME.to_string(),
            Some(CarriedKeyName::NS_URI.to_string()),
        );
        xml_object.add_namespace(
            CarriedKeyName::NS_PREFIX.to_string(),
            CarriedKeyName::NS_URI.to_string(),
        );
        xml_object.set_text(Some(carried_key_name.value.to_string()));
        Ok(xml_object)
    }
}
