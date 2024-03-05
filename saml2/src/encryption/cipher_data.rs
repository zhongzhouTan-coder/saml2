use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use crate::{common::SAML2Obj, error::SAMLError, xml::XmlObject};

use super::{cipher_reference::CipherReference, cipher_value::CipherValue};

#[derive(Default, Debug)]
pub struct CipherData {
    cipher_value: Option<CipherValue>,
    cipher_reference: Option<CipherReference>,
}

impl SAML2Obj for CipherData {}

impl CipherData {
    const CHILD_CIPHER_VALUE: &'static str = "CipherValue";
    const CHILD_CIPHER_REFERENCE: &'static str = "CipherReference";

    const ELEMENT_NAME: &'static str = "CipherData";
    const NS_PREFIX: &'static str = "xenc";
    const NS_URI: &'static str = "http://www.w3.org/2001/04/xmlenc#";

    #[inline]
    pub fn cipher_value(&self) -> Option<&CipherValue> {
        self.cipher_value.as_ref()
    }

    #[inline]
    pub fn set_cipher_value(&mut self, cipher_value: Option<CipherValue>) {
        self.cipher_value = cipher_value;
    }

    #[inline]
    pub fn cipher_reference(&self) -> Option<&CipherReference> {
        self.cipher_reference.as_ref()
    }

    #[inline]
    pub fn set_cipher_reference(&mut self, cipher_reference: Option<CipherReference>) {
        self.cipher_reference = cipher_reference;
    }
}

impl TryFrom<Ref<'_, XmlObject>> for CipherData {
    type Error = SAMLError;

    fn try_from(xml_object: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        let mut cipher_data = CipherData::default();
        for child in xml_object.children() {
            let child = child.borrow();
            match child.q_name().local_name() {
                CipherData::CHILD_CIPHER_VALUE => {
                    cipher_data.set_cipher_value(Some(CipherValue::try_from(child)?));
                }
                CipherData::CHILD_CIPHER_REFERENCE => {
                    cipher_data.set_cipher_reference(Some(CipherReference::try_from(child)?));
                }
                _ => {}
            }
        }
        Ok(cipher_data)
    }
}

impl TryFrom<CipherData> for XmlObject {
    type Error = SAMLError;

    fn try_from(cipher_data: CipherData) -> Result<Self, Self::Error> {
        let mut xml_obj = XmlObject::new(
            Some(CipherData::NS_PREFIX.to_string()),
            CipherData::ELEMENT_NAME.to_string(),
            Some(CipherData::NS_URI.to_string()),
        );
        xml_obj.add_namespace(
            CipherData::NS_PREFIX.to_string(),
            CipherData::NS_URI.to_string(),
        );
        if let Some(cipher_value) = cipher_data.cipher_value {
            xml_obj.add_child(Rc::new(RefCell::new(XmlObject::try_from(cipher_value)?)));
        }
        if let Some(cipher_reference) = cipher_data.cipher_reference {
            xml_obj.add_child(Rc::new(RefCell::new(XmlObject::try_from(
                cipher_reference,
            )?)));
        }
        Ok(xml_obj)
    }
}
