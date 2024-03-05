use std::cell::Ref;

use crate::{common::SAML2Obj, error::SAMLError, xml::XmlObject};

#[derive(Default, Debug)]
pub struct GetComplete {
    value: Option<String>,
}

impl SAML2Obj for GetComplete {}

impl GetComplete {
    const ELEMENT_NAME: &'static str = "GetComplete";
    const NS_PREFIX: &'static str = "saml2";
    const NS_URI: &'static str = "urn:oasis:names:tc:SAML:2.0:protocol";

    #[inline]
    pub fn value(&self) -> Option<&String> {
        self.value.as_ref()
    }

    #[inline]
    pub fn set_value(&mut self, value: Option<String>) {
        self.value = value;
    }
}

impl TryFrom<Ref<'_, XmlObject>> for GetComplete {
    type Error = SAMLError;

    fn try_from(object: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        match object.text() {
            Some(value) => Ok(GetComplete {
                value: Some(value.to_string()),
            }),
            None => Err(SAMLError::UnmarshallingError("Invalid XML".to_string())),
        }
    }
}

impl TryFrom<GetComplete> for XmlObject {
    type Error = SAMLError;

    fn try_from(get_complete: GetComplete) -> Result<Self, Self::Error> {
        let mut xml_object = XmlObject::new(
            Some(GetComplete::NS_PREFIX.to_string()),
            GetComplete::ELEMENT_NAME.to_string(),
            Some(GetComplete::NS_URI.to_string()),
        );
        xml_object.add_namespace(
            GetComplete::NS_PREFIX.to_string(),
            GetComplete::NS_URI.to_string(),
        );
        xml_object.set_text(get_complete.value);
        Ok(xml_object)
    }
}
