use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use crate::{
    common::SAML2Obj,
    encryption::{encrypted_data::EncryptedData, encrypted_key::EncryptedKey},
    error::SAMLError,
    xml::XmlObject,
};

#[derive(Debug, Default)]
pub struct EncryptedID {
    encrypted_data: EncryptedData,
    encrypted_keys: Vec<EncryptedKey>,
}

impl SAML2Obj for EncryptedID {}

impl EncryptedID {
    const CHILD_ENCRYPTED_DATA: &'static str = "EncryptedData";
    const CHILD_ENCRYPTED_KEY: &'static str = "EncryptedKey";

    const ELEMENT_NAME: &'static str = "EncryptedID";
    const NS_PREFIX: &'static str = "saml2";
    const NS_URI: &'static str = "urn:oasis:names:tc:SAML:2.0:assertion";

    #[inline]
    pub fn encrypted_data(&self) -> &EncryptedData {
        &self.encrypted_data
    }

    #[inline]
    pub fn set_encrypted_data(&mut self, encrypted_data: EncryptedData) {
        self.encrypted_data = encrypted_data;
    }

    #[inline]
    pub fn encrypted_keys(&self) -> &Vec<EncryptedKey> {
        self.encrypted_keys.as_ref()
    }

    #[inline]
    pub fn add_encrypted_key(&mut self, encrypted_key: EncryptedKey) {
        self.encrypted_keys.push(encrypted_key)
    }
}

impl TryFrom<Ref<'_, XmlObject>> for EncryptedID {
    type Error = SAMLError;

    fn try_from(element: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        let mut encrypted_id = EncryptedID::default();
        for child in element.children() {
            let child = child.borrow();
            match child.q_name().local_name() {
                EncryptedID::CHILD_ENCRYPTED_DATA => {
                    encrypted_id.set_encrypted_data(EncryptedData::try_from(child)?);
                }
                EncryptedID::CHILD_ENCRYPTED_KEY => {
                    encrypted_id.add_encrypted_key(EncryptedKey::try_from(child)?);
                }
                _ => {}
            }
        }
        Ok(encrypted_id)
    }
}

impl TryFrom<EncryptedID> for XmlObject {
    type Error = SAMLError;

    fn try_from(encrypted_id: EncryptedID) -> Result<Self, Self::Error> {
        let mut xml_obj = XmlObject::new(
            Some(EncryptedID::NS_PREFIX.to_string()),
            EncryptedID::ELEMENT_NAME.to_string(),
            Some(EncryptedID::NS_URI.to_string()),
        );
        xml_obj.add_namespace(
            EncryptedID::NS_PREFIX.to_string(),
            EncryptedID::NS_URI.to_string(),
        );
        xml_obj.add_child(Rc::new(RefCell::new(XmlObject::try_from(
            encrypted_id.encrypted_data,
        )?)));
        for encrypted_key in encrypted_id.encrypted_keys {
            xml_obj.add_child(Rc::new(RefCell::new(XmlObject::try_from(encrypted_key)?)));
        }
        Ok(xml_obj)
    }
}
