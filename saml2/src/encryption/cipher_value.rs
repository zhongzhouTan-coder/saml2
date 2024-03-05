use std::cell::Ref;

use crate::{common::SAML2Obj, error::SAMLError, xml::XmlObject};

#[derive(Default, Debug)]
pub struct CipherValue {
    value: String,
}

impl SAML2Obj for CipherValue {}

impl CipherValue {
    const ELEMENT_NAME: &'static str = "CipherValue";

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

impl TryFrom<Ref<'_, XmlObject>> for CipherValue {
    type Error = SAMLError;

    fn try_from(cipher_value: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        match cipher_value.text() {
            Some(value) => Ok(CipherValue {
                value: value.to_string(),
            }),
            None => Err(SAMLError::UnmarshallingError("Invalid XML".to_string())),
        }
    }
}

impl TryFrom<CipherValue> for XmlObject {
    type Error = SAMLError;

    fn try_from(cipher_value: CipherValue) -> Result<Self, Self::Error> {
        let mut xml_object = XmlObject::new(
            Some(CipherValue::NS_PREFIX.to_string()),
            CipherValue::ELEMENT_NAME.to_string(),
            Some(CipherValue::NS_URI.to_string()),
        );
        xml_object.add_namespace(
            CipherValue::NS_PREFIX.to_string(),
            CipherValue::NS_URI.to_string(),
        );
        xml_object.set_text(Some(cipher_value.value.to_string()));
        Ok(xml_object)
    }
}
