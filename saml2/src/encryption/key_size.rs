use std::cell::Ref;

use crate::{common::SAML2Obj, error::SAMLError, xml::XmlObject};

#[derive(Debug, Default)]
pub struct KeySize {
    value: u32,
}

impl SAML2Obj for KeySize {}

impl KeySize {
    const ELEMENT_NAME: &'static str = "KeySize";
    const NS_PREFIX: &'static str = "xenc";
    const NS_URI: &'static str = "http://www.w3.org/2001/04/xmlenc#";

    #[inline]
    pub fn value(&self) -> u32 {
        self.value
    }

    #[inline]
    pub fn set_value(&mut self, value: u32) {
        self.value = value;
    }
}

impl TryFrom<Ref<'_, XmlObject>> for KeySize {
    type Error = SAMLError;

    fn try_from(key_size: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        match key_size.text() {
            Some(key_size) => Ok(KeySize {
                value: key_size
                    .parse()
                    .map_err(|_| SAMLError::UnmarshallingError("Invalid KeySize".to_string()))?,
            }),
            None => Err(SAMLError::UnmarshallingError("Invalid XML".to_string())),
        }
    }
}

impl TryFrom<KeySize> for XmlObject {
    type Error = SAMLError;

    fn try_from(key_size: KeySize) -> Result<Self, Self::Error> {
        let mut xml_object = XmlObject::new(
            Some(KeySize::ELEMENT_NAME.to_string()),
            KeySize::NS_PREFIX.to_string(),
            Some(KeySize::NS_URI.to_string()),
        );
        xml_object.set_text(Some(key_size.value.to_string()));
        Ok(xml_object)
    }
}
