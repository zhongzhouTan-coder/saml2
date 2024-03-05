use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use crate::{common::SAML2Obj, error::SAMLError, xml::XmlObject};

use super::{key_size::KeySize, oaep_params::OAEPParams};

#[derive(Default, Debug)]
pub struct EncryptedMethod {
    algorithm: String,
    key_size: KeySize,
    oeap_params: OAEPParams,
    unknown_children: Vec<Box<dyn SAML2Obj>>,
}

impl SAML2Obj for EncryptedMethod {}

impl EncryptedMethod {
    const ATTRIBUTE_ALGORITHM: &'static str = "Algorithm";

    const CHILD_KEY_SIZE: &'static str = "KeySize";
    const CHILD_OEAP_PARAMS: &'static str = "OAEPParams";

    const ELEMENT_NAME: &'static str = "EncryptedMethod";
    const NS_PREFIX: &'static str = "xenc";
    const NS_URI: &'static str = "http://www.w3.org/2001/04/xmlenc#";

    #[inline]
    pub fn algorithm(&self) -> &str {
        &self.algorithm
    }

    #[inline]
    pub fn set_algorithm(&mut self, algorithm: String) {
        self.algorithm = algorithm;
    }

    #[inline]
    pub fn key_size(&self) -> &KeySize {
        &self.key_size
    }

    #[inline]
    pub fn set_key_size(&mut self, key_size: KeySize) {
        self.key_size = key_size;
    }

    #[inline]
    pub fn oeap_params(&self) -> &OAEPParams {
        &self.oeap_params
    }

    #[inline]
    pub fn set_oeap_params(&mut self, oeap_params: OAEPParams) {
        self.oeap_params = oeap_params;
    }

    #[inline]
    pub fn unknown_children(&self) -> &Vec<Box<dyn SAML2Obj>> {
        &self.unknown_children
    }

    #[inline]
    pub fn add_unknown_child(&mut self, unknown_child: Box<dyn SAML2Obj>) {
        self.unknown_children.push(unknown_child);
    }
}

impl TryFrom<Ref<'_, XmlObject>> for EncryptedMethod {
    type Error = SAMLError;

    fn try_from(value: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        let mut encrypted_method = EncryptedMethod::default();
        for attr in value.attributes() {
            match attr.0.as_str() {
                EncryptedMethod::ATTRIBUTE_ALGORITHM => {
                    encrypted_method.set_algorithm(attr.1.to_string());
                }
                _ => {}
            }
        }
        for child in value.children() {
            let child = child.borrow();
            match child.q_name().local_name() {
                EncryptedMethod::CHILD_KEY_SIZE => {
                    todo!("KeySize not implemented yet")
                }
                EncryptedMethod::CHILD_OEAP_PARAMS => {
                    todo!("OAEPParams not implemented yet")
                }
                _ => {
                    todo!("Unknown child not implemented yet")
                }
            }
        }
        Ok(encrypted_method)
    }
}

impl TryFrom<EncryptedMethod> for XmlObject {
    type Error = SAMLError;

    fn try_from(encrypted_method: EncryptedMethod) -> Result<Self, Self::Error> {
        let mut xml_obj = XmlObject::new(
            Some(EncryptedMethod::NS_URI.to_string()),
            EncryptedMethod::ELEMENT_NAME.to_string(),
            Some(EncryptedMethod::NS_PREFIX.to_string()),
        );
        xml_obj.add_namespace(
            EncryptedMethod::NS_PREFIX.to_string(),
            EncryptedMethod::NS_URI.to_string(),
        );
        xml_obj.add_attribute(
            EncryptedMethod::ATTRIBUTE_ALGORITHM.to_string(),
            encrypted_method.algorithm().to_string(),
        );
        xml_obj.add_child(Rc::new(RefCell::new(XmlObject::try_from(
            encrypted_method.key_size,
        )?)));
        xml_obj.add_child(Rc::new(RefCell::new(XmlObject::try_from(
            encrypted_method.oeap_params,
        )?)));
        for unknown_child in encrypted_method.unknown_children {
            todo!("Implement conversion from EncryptedMethod to XmlObject")
        }
        Ok(xml_obj)
    }
}
