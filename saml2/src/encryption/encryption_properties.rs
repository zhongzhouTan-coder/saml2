use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use crate::{common::SAML2Obj, error::SAMLError, xml::XmlObject};

use super::encryption_property::EncryptionProperty;

#[derive(Default, Debug)]
pub struct EncryptionProperties {
    id: String,
    encryption_properties: Vec<EncryptionProperty>,
}

impl SAML2Obj for EncryptionProperties {}

impl EncryptionProperties {
    const ATTRIBUTE_ID: &'static str = "Id";

    const CHILD_ENCRYPTION_PROPERTY: &'static str = "EncryptionProperty";

    const ELEMENT_NAME: &'static str = "EncryptionProperties";
    const NS_PREFIX: &'static str = "xenc";
    const NS_URI: &'static str = "http://www.w3.org/2001/04/xmlenc#";

    #[inline]
    pub fn id(&self) -> &str {
        &self.id
    }

    #[inline]
    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    #[inline]
    pub fn encryption_properties(&self) -> &Vec<EncryptionProperty> {
        &self.encryption_properties
    }

    #[inline]
    pub fn add_encryption_property(&mut self, encryption_property: EncryptionProperty) {
        self.encryption_properties.push(encryption_property);
    }
}

impl TryFrom<Ref<'_, XmlObject>> for EncryptionProperties {
    type Error = SAMLError;

    fn try_from(encryption_properties: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        let mut enc_props = EncryptionProperties::default();
        for child in encryption_properties.children() {
            let child = child.borrow();
            match child.q_name().local_name() {
                EncryptionProperties::CHILD_ENCRYPTION_PROPERTY => {
                    enc_props.add_encryption_property(EncryptionProperty::try_from(child)?);
                }
                _ => {}
            }
        }
        Ok(enc_props)
    }
}

impl TryFrom<EncryptionProperties> for XmlObject {
    type Error = SAMLError;

    fn try_from(enc_props: EncryptionProperties) -> Result<Self, Self::Error> {
        let mut xml_obj = XmlObject::new(
            Some(EncryptionProperties::NS_PREFIX.to_string()),
            EncryptionProperties::ELEMENT_NAME.to_string(),
            Some(EncryptionProperties::NS_URI.to_string()),
        );
        xml_obj.add_namespace(
            EncryptionProperties::NS_PREFIX.to_string(),
            EncryptionProperties::NS_URI.to_string(),
        );
        xml_obj.add_attribute(EncryptionProperties::ATTRIBUTE_ID.to_string(), enc_props.id);
        for enc_prop in enc_props.encryption_properties {
            xml_obj.add_child(Rc::new(RefCell::new(XmlObject::try_from(enc_prop)?)));
        }
        Ok(xml_obj)
    }
}
