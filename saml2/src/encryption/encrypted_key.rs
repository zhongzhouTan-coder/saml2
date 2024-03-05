use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use crate::{common::SAML2Obj, error::SAMLError, signature::key_info::KeyInfo, xml::XmlObject};

use super::{
    carried_key_name::CarriedKeyName, cipher_data::CipherData, encrypted_method::EncryptedMethod,
    encryption_properties::EncryptionProperties, reference_list::ReferenceList,
};

#[derive(Default, Debug)]
pub struct EncryptedKey {
    id: Option<String>,
    encrypted_type: Option<String>,
    mime_type: Option<String>,
    encoding: Option<String>,
    encrypted_method: Option<EncryptedMethod>,
    key_info: Option<KeyInfo>,
    cipher_data: CipherData,
    encryption_properties: Option<EncryptionProperties>,
    recipient: Option<String>,
    carried_key_name: Option<CarriedKeyName>,
    reference_list: Option<ReferenceList>,
}

impl SAML2Obj for EncryptedKey {}

impl EncryptedKey {
    const ELEMENT_NAME: &'static str = "EncryptedKey";
    const NS_PREFIX: &'static str = "xenc";
    const NS_URI: &'static str = "http://www.w3.org/2001/04/xmlenc#";

    const ATTRIB_RECIPIENT: &'static str = "Recipient";
    const ATTRIB_ID: &'static str = "Id";
    const ATTRIB_ENCRYPTED_TYPE: &'static str = "Type";
    const ATTRIB_MIME_TYPE: &'static str = "MimeType";
    const ATTRIB_ENCODING: &'static str = "Encoding";

    const CHILD_ENCRYPTED_METHOD: &'static str = "EncryptionMethod";
    const CHILD_KEY_INFO: &'static str = "KeyInfo";
    const CHILD_CIPHER_DATA: &'static str = "CipherData";
    const CHILD_ENCRYPTION_PROPERTIES: &'static str = "EncryptionProperties";
    const CHILD_CARRIED_KEY_NAME: &'static str = "CarriedKeyName";
    const CHILD_REFERENCE_LIST: &'static str = "ReferenceList";

    #[inline]
    pub fn id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    #[inline]
    pub fn set_id(&mut self, id: Option<String>) {
        self.id = id;
    }

    #[inline]
    pub fn encrypted_type(&self) -> Option<&String> {
        self.encrypted_type.as_ref()
    }

    #[inline]
    pub fn set_encrypted_type(&mut self, encrypted_type: Option<String>) {
        self.encrypted_type = encrypted_type;
    }

    #[inline]
    pub fn mime_type(&self) -> Option<&String> {
        self.mime_type.as_ref()
    }

    #[inline]
    pub fn set_mime_type(&mut self, mime_type: Option<String>) {
        self.mime_type = mime_type;
    }

    #[inline]
    pub fn encoding(&self) -> Option<&String> {
        self.encoding.as_ref()
    }

    #[inline]
    pub fn set_encoding(&mut self, encoding: Option<String>) {
        self.encoding = encoding;
    }

    #[inline]
    pub fn encrypted_method(&self) -> Option<&EncryptedMethod> {
        self.encrypted_method.as_ref()
    }

    #[inline]
    pub fn set_encrypted_method(&mut self, encrypted_method: Option<EncryptedMethod>) {
        self.encrypted_method = encrypted_method;
    }

    #[inline]
    pub fn key_info(&self) -> Option<&KeyInfo> {
        self.key_info.as_ref()
    }

    #[inline]
    pub fn set_key_info(&mut self, key_info: Option<KeyInfo>) {
        self.key_info = key_info;
    }

    #[inline]
    pub fn cipher_data(&self) -> &CipherData {
        &self.cipher_data
    }

    #[inline]
    pub fn set_cipher_data(&mut self, cipher_data: CipherData) {
        self.cipher_data = cipher_data;
    }

    #[inline]
    pub fn encryption_properties(&self) -> Option<&EncryptionProperties> {
        self.encryption_properties.as_ref()
    }

    #[inline]
    pub fn set_encryption_properties(
        &mut self,
        encryption_properties: Option<EncryptionProperties>,
    ) {
        self.encryption_properties = encryption_properties;
    }

    #[inline]
    pub fn recipient(&self) -> Option<&String> {
        self.recipient.as_ref()
    }

    #[inline]
    pub fn set_recipient(&mut self, recipient: Option<String>) {
        self.recipient = recipient;
    }

    #[inline]
    pub fn carried_key_name(&self) -> Option<&CarriedKeyName> {
        self.carried_key_name.as_ref()
    }

    #[inline]
    pub fn set_carried_key_name(&mut self, carried_key_name: Option<CarriedKeyName>) {
        self.carried_key_name = carried_key_name;
    }

    #[inline]
    pub fn reference_list(&self) -> Option<&ReferenceList> {
        self.reference_list.as_ref()
    }

    #[inline]
    pub fn set_reference_list(&mut self, reference_list: Option<ReferenceList>) {
        self.reference_list = reference_list;
    }
}

impl TryFrom<Ref<'_, XmlObject>> for EncryptedKey {
    type Error = SAMLError;

    fn try_from(encrypted_key: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        let mut enc_key = EncryptedKey::default();
        for attrib in encrypted_key.attributes() {
            match attrib.0.as_str() {
                EncryptedKey::ATTRIB_ID => {
                    enc_key.set_id(Some(attrib.1.to_string()));
                }
                EncryptedKey::ATTRIB_ENCRYPTED_TYPE => {
                    enc_key.set_encrypted_type(Some(attrib.1.to_string()));
                }
                EncryptedKey::ATTRIB_MIME_TYPE => {
                    enc_key.set_mime_type(Some(attrib.1.to_string()));
                }
                EncryptedKey::ATTRIB_ENCODING => {
                    enc_key.set_encoding(Some(attrib.1.to_string()));
                }
                EncryptedKey::ATTRIB_RECIPIENT => {
                    enc_key.set_recipient(Some(attrib.1.to_string()));
                }
                _ => {}
            }
        }
        for child in encrypted_key.children() {
            let child = child.borrow();
            match child.q_name().local_name() {
                EncryptedKey::CHILD_ENCRYPTED_METHOD => {
                    enc_key.set_encrypted_method(Some(EncryptedMethod::try_from(child)?));
                }
                EncryptedKey::CHILD_KEY_INFO => {
                    enc_key.set_key_info(Some(KeyInfo::try_from(child)?));
                }
                EncryptedKey::CHILD_CIPHER_DATA => {
                    enc_key.set_cipher_data(CipherData::try_from(child)?);
                }
                EncryptedKey::CHILD_ENCRYPTION_PROPERTIES => {
                    enc_key.set_encryption_properties(Some(EncryptionProperties::try_from(child)?));
                }
                EncryptedKey::CHILD_CARRIED_KEY_NAME => {
                    enc_key.set_carried_key_name(Some(CarriedKeyName::try_from(child)?));
                }
                EncryptedKey::CHILD_REFERENCE_LIST => {
                    enc_key.set_reference_list(Some(ReferenceList::try_from(child)?));
                }
                _ => {}
            }
        }
        Ok(enc_key)
    }
}

impl TryFrom<EncryptedKey> for XmlObject {
    type Error = SAMLError;

    fn try_from(enc_key: EncryptedKey) -> Result<Self, Self::Error> {
        let mut xml_obj = XmlObject::new(
            Some(EncryptedKey::NS_PREFIX.to_string()),
            EncryptedKey::ELEMENT_NAME.to_string(),
            Some(EncryptedKey::NS_URI.to_string()),
        );
        xml_obj.add_namespace(
            EncryptedKey::NS_PREFIX.to_string(),
            EncryptedKey::NS_URI.to_string(),
        );
        if let Some(id) = enc_key.id {
            xml_obj.add_attribute(EncryptedKey::ATTRIB_ID.to_string(), id);
        }
        if let Some(encrypted_type) = enc_key.encrypted_type {
            xml_obj.add_attribute(
                EncryptedKey::ATTRIB_ENCRYPTED_TYPE.to_string(),
                encrypted_type,
            );
        }
        if let Some(mime_type) = enc_key.mime_type {
            xml_obj.add_attribute(EncryptedKey::ATTRIB_MIME_TYPE.to_string(), mime_type);
        }
        if let Some(encoding) = enc_key.encoding {
            xml_obj.add_attribute(EncryptedKey::ATTRIB_ENCODING.to_string(), encoding);
        }
        if let Some(recipient) = enc_key.recipient {
            xml_obj.add_attribute(EncryptedKey::ATTRIB_RECIPIENT.to_string(), recipient);
        }
        if let Some(encrypted_method) = enc_key.encrypted_method {
            xml_obj.add_child(Rc::new(RefCell::new(XmlObject::try_from(
                encrypted_method,
            )?)));
        }
        if let Some(key_info) = enc_key.key_info {
            xml_obj.add_child(Rc::new(RefCell::new(XmlObject::try_from(key_info)?)));
        }
        xml_obj.add_child(Rc::new(RefCell::new(XmlObject::try_from(
            enc_key.cipher_data,
        )?)));
        if let Some(encryption_properties) = enc_key.encryption_properties {
            xml_obj.add_child(Rc::new(RefCell::new(XmlObject::try_from(
                encryption_properties,
            )?)));
        }
        if let Some(carried_key_name) = enc_key.carried_key_name {
            xml_obj.add_child(Rc::new(RefCell::new(XmlObject::try_from(
                carried_key_name,
            )?)));
        }
        if let Some(reference_list) = enc_key.reference_list {
            xml_obj.add_child(Rc::new(RefCell::new(XmlObject::try_from(reference_list)?)));
        }
        Ok(xml_obj)
    }
}
