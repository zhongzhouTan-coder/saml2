use std::cell::Ref;

use crate::{common::SAML2Obj, error::SAMLError, xml::XmlObject};

#[derive(Debug, Default)]
pub struct Extensions {
    unknown_children: Vec<Box<dyn SAML2Obj>>,
}

impl SAML2Obj for Extensions {}

impl Extensions {
    const ELEMENT_NAME: &'static str = "Extensions";
    const NS_PREFIX: &'static str = "samlp";
    const NS_URI: &'static str = "urn:oasis:names:tc:SAML:2.0:protocol";

    #[inline]
    pub fn unknown_children(&self) -> &Vec<Box<dyn SAML2Obj>> {
        &self.unknown_children
    }
}

impl TryFrom<Ref<'_, XmlObject>> for Extensions {
    type Error = SAMLError;

    fn try_from(xml_obj: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        let mut extensions = Extensions::default();
        for child in xml_obj.children() {
            let child = child.borrow();
            match child.q_name().local_name() {
                _ => {
                    todo!("Extensions::try_from<Ref<XmlObject>>")
                }
            }
        }
        Ok(extensions)
    }
}

impl TryFrom<Extensions> for XmlObject {
    type Error = SAMLError;

    fn try_from(extensions: Extensions) -> Result<Self, Self::Error> {
        let mut xml_obj = XmlObject::new(
            Some(Extensions::NS_PREFIX.to_string()),
            Extensions::ELEMENT_NAME.to_string(),
            Some(Extensions::NS_URI.to_string()),
        );
        xml_obj.add_namespace(
            Extensions::NS_PREFIX.to_string(),
            Extensions::NS_URI.to_string(),
        );
        for child in extensions.unknown_children {
            todo!("XmlObject::try_from<dyn SAML2Obj>")
        }
        Ok(xml_obj)
    }
}
