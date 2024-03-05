use std::cell::Ref;

use crate::{common::SAML2Obj, error::SAMLError, xml::XmlObject};

#[derive(Debug, Default)]
pub struct ReferenceList {
    indexed_children: Vec<Box<dyn SAML2Obj>>,
}

impl SAML2Obj for ReferenceList {}

impl ReferenceList {
    const ELEMENT_NAME: &'static str = "ReferenceList";
    const NS_PREFIX: &'static str = "xenc";
    const NS_URI: &'static str = "http://www.w3.org/2001/04/xmlenc#";

    #[inline]
    pub fn indexed_children(&self) -> &Vec<Box<dyn SAML2Obj>> {
        &self.indexed_children
    }
}

impl TryFrom<Ref<'_, XmlObject>> for ReferenceList {
    type Error = SAMLError;

    fn try_from(reference_list: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        let mut ref_list = ReferenceList::default();
        for child in reference_list.children() {
            todo!("Try from for child is not implemented yet.")
        }
        Ok(ref_list)
    }
}

impl TryFrom<ReferenceList> for XmlObject {
    type Error = SAMLError;

    fn try_from(reference_list: ReferenceList) -> Result<XmlObject, Self::Error> {
        let mut xml_obj = XmlObject::new(
            Some(ReferenceList::NS_PREFIX.to_string()),
            ReferenceList::ELEMENT_NAME.to_string(),
            Some(ReferenceList::NS_URI.to_string()),
        );
        for child in reference_list.indexed_children() {
            todo!("Try from for child is not implemented yet.")
        }
        Ok(xml_obj)
    }
}
