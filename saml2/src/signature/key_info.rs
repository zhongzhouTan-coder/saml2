use std::cell::Ref;

use crate::{common::SAML2Obj, error::SAMLError, xml::XmlObject};

#[derive(Default, Debug)]
pub struct KeyInfo {
    id: Option<String>,
    indexed_children: Vec<Box<dyn SAML2Obj>>,
}

impl SAML2Obj for KeyInfo {}

impl KeyInfo {
    const ATTRIBUTE_ID: &'static str = "Id";

    const ELEMENT_NAME: &'static str = "KeyInfo";
    const NS_PREFIX: &'static str = "ds";
    const NS_URI: &'static str = "http://www.w3.org/2000/09/xmldsig#";

    #[inline]
    pub fn id(&self) -> &Option<String> {
        &self.id
    }

    #[inline]
    pub fn set_id(&mut self, id: Option<String>) {
        self.id = id;
    }

    #[inline]
    pub fn indexed_children(&self) -> &Vec<Box<dyn SAML2Obj>> {
        &self.indexed_children
    }

    #[inline]
    pub fn add_indexed_child(&mut self, child: Box<dyn SAML2Obj>) {
        self.indexed_children.push(child);
    }
}

impl TryFrom<Ref<'_, XmlObject>> for KeyInfo {
    type Error = SAMLError;

    fn try_from(value: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        let mut key_info = KeyInfo::default();
        for attr in value.attributes() {
            match attr.0.as_str() {
                KeyInfo::ATTRIBUTE_ID => {
                    key_info.set_id(Some(attr.1.to_string()));
                }
                _ => {}
            }
        }
        for child in value.children() {
            let child = child.borrow();
            todo!("Implement conversion from XmlObject to KeyInfo")
        }
        Ok(key_info)
    }
}

impl TryFrom<KeyInfo> for XmlObject {
    type Error = SAMLError;

    fn try_from(key_info: KeyInfo) -> Result<Self, Self::Error> {
        let mut xml_object = XmlObject::new(
            Some(KeyInfo::NS_PREFIX.to_string()),
            KeyInfo::ELEMENT_NAME.to_string(),
            Some(KeyInfo::NS_URI.to_string()),
        );
        xml_object.add_namespace(KeyInfo::NS_PREFIX.to_string(), KeyInfo::NS_URI.to_string());
        if let Some(id) = key_info.id {
            xml_object.add_attribute(KeyInfo::ATTRIBUTE_ID.to_string(), id);
        }
        for child in key_info.indexed_children {
            todo!("Implement conversion from KeyInfo to XmlObject")
        }
        Ok(xml_object)
    }
}
