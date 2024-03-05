use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use crate::{common::SAML2Obj, error::SAMLError, xml::XmlObject};

use super::transforms::Transforms;

#[derive(Default, Debug)]
pub struct CipherReference {
    uri: Option<String>,
    transforms: Option<Transforms>,
}

impl SAML2Obj for CipherReference {}

impl CipherReference {
    const ATTRIB_URI: &'static str = "URI";
    const CHILD_TRANSFORMS: &'static str = "Transforms";

    const ELEMENT_NAME: &'static str = "CipherReference";
    const NS_PREFIX: &'static str = "xenc";
    const NS_URI: &'static str = "http://www.w3.org/2001/04/xmlenc#";

    #[inline]
    pub fn uri(&self) -> Option<&String> {
        self.uri.as_ref()
    }

    #[inline]
    pub fn set_uri(&mut self, uri: Option<String>) {
        self.uri = uri;
    }

    #[inline]
    pub fn transforms(&self) -> Option<&Transforms> {
        self.transforms.as_ref()
    }

    #[inline]
    pub fn set_transforms(&mut self, transforms: Option<Transforms>) {
        self.transforms = transforms;
    }
}

impl TryFrom<Ref<'_, XmlObject>> for CipherReference {
    type Error = SAMLError;

    fn try_from(cipher_reference: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        let mut cipher_ref = CipherReference::default();
        for attrib in cipher_reference.attributes() {
            match attrib.0.as_str() {
                CipherReference::ATTRIB_URI => {
                    cipher_ref.set_uri(Some(attrib.1.to_string()));
                }
                _ => {}
            }
        }
        for child in cipher_reference.children() {
            let child = child.borrow();
            match child.q_name().local_name() {
                CipherReference::CHILD_TRANSFORMS => {
                    cipher_ref.set_transforms(Some(Transforms::try_from(child)?));
                }
                _ => {}
            }
        }
        Ok(cipher_ref)
    }
}

impl TryFrom<CipherReference> for XmlObject {
    type Error = SAMLError;

    fn try_from(cipher_ref: CipherReference) -> Result<Self, Self::Error> {
        let mut xml_object = XmlObject::new(
            Some(CipherReference::NS_PREFIX.to_string()),
            CipherReference::ELEMENT_NAME.to_string(),
            Some(CipherReference::NS_URI.to_string()),
        );
        xml_object.add_namespace(
            CipherReference::NS_PREFIX.to_string(),
            CipherReference::NS_URI.to_string(),
        );
        if let Some(uri) = cipher_ref.uri {
            xml_object.add_attribute(CipherReference::ATTRIB_URI.to_string(), uri.to_string());
        }
        if let Some(transforms) = cipher_ref.transforms {
            xml_object.add_child(Rc::new(RefCell::new(XmlObject::try_from(transforms)?)));
        }
        Ok(xml_object)
    }
}
