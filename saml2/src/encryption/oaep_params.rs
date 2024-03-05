use std::cell::Ref;

use crate::{common::SAML2Obj, error::SAMLError, xml::XmlObject};

#[derive(Default, Debug)]
pub struct OAEPParams {
    value: Option<String>,
}

impl SAML2Obj for OAEPParams {}

impl OAEPParams {
    const ELEMENT_NAME: &'static str = "OAEPParams";
    const NS_PREFIX: &'static str = "xenc";
    const NS_URI: &'static str = "http://www.w3.org/2001/04/xmlenc#";

    #[inline]
    pub fn value(&self) -> Option<&String> {
        self.value.as_ref()
    }

    #[inline]
    pub fn set_value(&mut self, value: Option<String>) {
        self.value = value;
    }
}

impl TryFrom<Ref<'_, XmlObject>> for OAEPParams {
    type Error = SAMLError;

    fn try_from(value: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        let mut oaep_params = OAEPParams::default();
        oaep_params.set_value(value.text().map(|s| s.to_string()));
        Ok(oaep_params)
    }
}

impl TryFrom<OAEPParams> for XmlObject {
    type Error = SAMLError;

    fn try_from(oaep_params: OAEPParams) -> Result<XmlObject, Self::Error> {
        let mut xml_obj = XmlObject::new(
            Some(OAEPParams::NS_PREFIX.to_string()),
            OAEPParams::ELEMENT_NAME.to_string(),
            Some(OAEPParams::NS_URI.to_string()),
        );
        if let Some(value) = oaep_params.value() {
            xml_obj.set_text(Some(value.to_string()));
        }
        Ok(xml_obj)
    }
}
