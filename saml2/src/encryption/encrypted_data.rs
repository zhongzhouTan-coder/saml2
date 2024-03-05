use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use crate::{common::SAML2Obj, error::SAMLError, signature::key_info::KeyInfo, xml::XmlObject};

use super::{
    cipher_data::CipherData, encrypted_method::EncryptedMethod,
    encryption_properties::EncryptionProperties,
};

#[derive(Default, Debug)]
pub struct EncryptedData {
    id: Option<String>,
    encrypted_type: Option<String>,
    mime_type: Option<String>,
    encoding: Option<String>,
    encrypted_method: Option<EncryptedMethod>,
    key_info: Option<KeyInfo>,
    cipher_data: CipherData,
    encryption_properties: Option<EncryptionProperties>,
}

impl SAML2Obj for EncryptedData {}

impl EncryptedData {
    const ATTRIB_ID: &'static str = "Id";
    const ATTRIB_ENCRYPTED_TYPE: &'static str = "Type";
    const ATTRIB_MIME_TYPE: &'static str = "MimeType";
    const ATTRIB_ENCODING: &'static str = "Encoding";

    const CHILD_ENCRYPTED_METHOD: &'static str = "EncryptionMethod";
    const CHILD_KEY_INFO: &'static str = "KeyInfo";
    const CHILD_CIPHER_DATA: &'static str = "CipherData";
    const CHILD_ENCRYPTION_PROPERTIES: &'static str = "EncryptionProperties";

    const ELEMENT_NAME: &'static str = "EncryptedData";
    const NS_PREFIX: &'static str = "xenc";
    const NS_URI: &'static str = "http://www.w3.org/2001/04/xmlenc#";

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
}

impl TryFrom<Ref<'_, XmlObject>> for EncryptedData {
    type Error = SAMLError;

    fn try_from(value: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        let mut encrypted_data = EncryptedData::default();

        for attrib in value.attributes() {
            match attrib.0.as_str() {
                EncryptedData::ATTRIB_ID => {
                    encrypted_data.set_id(Some(attrib.1.to_string()));
                }
                EncryptedData::ATTRIB_ENCRYPTED_TYPE => {
                    encrypted_data.set_encrypted_type(Some(attrib.1.to_string()));
                }
                EncryptedData::ATTRIB_MIME_TYPE => {
                    encrypted_data.set_mime_type(Some(attrib.1.to_string()));
                }
                EncryptedData::ATTRIB_ENCODING => {
                    encrypted_data.set_encoding(Some(attrib.1.to_string()));
                }
                _ => {}
            }
        }
        for child in value.children() {
            let child = child.borrow();
            match child.q_name().local_name() {
                EncryptedData::CHILD_ENCRYPTED_METHOD => {
                    encrypted_data.set_encrypted_method(Some(EncryptedMethod::try_from(child)?));
                }
                EncryptedData::CHILD_KEY_INFO => {
                    encrypted_data.set_key_info(Some(KeyInfo::try_from(child)?));
                }
                EncryptedData::CHILD_CIPHER_DATA => {
                    encrypted_data.set_cipher_data(CipherData::try_from(child)?);
                }
                EncryptedData::CHILD_ENCRYPTION_PROPERTIES => {
                    encrypted_data
                        .set_encryption_properties(Some(EncryptionProperties::try_from(child)?));
                }
                _ => {}
            }
        }
        Ok(encrypted_data)
    }
}

impl TryFrom<EncryptedData> for XmlObject {
    type Error = SAMLError;

    fn try_from(encrypted_data: EncryptedData) -> Result<Self, Self::Error> {
        let mut xml_object = XmlObject::new(
            Some(EncryptedData::NS_PREFIX.to_string()),
            EncryptedData::ELEMENT_NAME.to_string(),
            Some(EncryptedData::NS_URI.to_string()),
        );
        xml_object.add_namespace(
            EncryptedData::NS_PREFIX.to_string(),
            EncryptedData::NS_URI.to_string(),
        );
        if let Some(id) = encrypted_data.id {
            xml_object.add_attribute(EncryptedData::ATTRIB_ID.to_string(), id);
        }
        if let Some(encrypted_type) = encrypted_data.encrypted_type {
            xml_object.add_attribute(
                EncryptedData::ATTRIB_ENCRYPTED_TYPE.to_string(),
                encrypted_type,
            );
        }
        if let Some(mime_type) = encrypted_data.mime_type {
            xml_object.add_attribute(EncryptedData::ATTRIB_MIME_TYPE.to_string(), mime_type);
        }
        if let Some(encoding) = encrypted_data.encoding {
            xml_object.add_attribute(EncryptedData::ATTRIB_ENCODING.to_string(), encoding);
        }
        if let Some(encrypted_method) = encrypted_data.encrypted_method {
            xml_object.add_child(Rc::new(RefCell::new(XmlObject::try_from(
                encrypted_method,
            )?)));
        }
        if let Some(key_info) = encrypted_data.key_info {
            xml_object.add_child(Rc::new(RefCell::new(XmlObject::try_from(key_info)?)));
        }
        xml_object.add_child(Rc::new(RefCell::new(XmlObject::try_from(
            encrypted_data.cipher_data,
        )?)));
        if let Some(encryption_properties) = encrypted_data.encryption_properties {
            xml_object.add_child(Rc::new(RefCell::new(XmlObject::try_from(
                encryption_properties,
            )?)));
        }
        Ok(xml_object)
    }
}
