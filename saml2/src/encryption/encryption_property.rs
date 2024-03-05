use std::cell::Ref;

use crate::{common::SAML2Obj, error::SAMLError, util::AttributeMap, xml::XmlObject};

#[derive(Default, Debug)]
pub struct EncryptionProperty {
    target: String,
    id: String,
    unknown_children: Vec<Box<dyn SAML2Obj>>,
    unknown_attributes: AttributeMap,
}

impl SAML2Obj for EncryptionProperty {}

impl EncryptionProperty {
    const ATTRIBUTE_TARGET: &'static str = "Target";
    const ATTRIBUTE_ID: &'static str = "ID";

    const ELEMENT_NAME: &'static str = "EncryptionProperty";
    const NS_PREFIX: &'static str = "xenc";
    const NS_URI: &'static str = "http://www.w3.org/2001/04/xmlenc#";

    #[inline]
    pub fn target(&self) -> &str {
        &self.target
    }

    #[inline]
    pub fn set_target(&mut self, target: String) {
        self.target = target;
    }

    #[inline]
    pub fn id(&self) -> &str {
        &self.id
    }

    #[inline]
    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    #[inline]
    pub fn unknown_children(&self) -> &Vec<Box<dyn SAML2Obj>> {
        &self.unknown_children
    }

    #[inline]
    pub fn unknown_attributes(&self) -> &AttributeMap {
        &self.unknown_attributes
    }
}

impl TryFrom<Ref<'_, XmlObject>> for EncryptionProperty {
    type Error = SAMLError;

    fn try_from(encryption_property: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        let mut enc_prop = EncryptionProperty::default();
        for attrib in encryption_property.attributes() {
            match attrib.0.as_str() {
                EncryptionProperty::ATTRIBUTE_TARGET => {
                    enc_prop.set_target(attrib.1.to_string());
                }
                EncryptionProperty::ATTRIBUTE_ID => {
                    enc_prop.set_id(attrib.1.to_string());
                }
                _ => {
                    todo!("Handle unknown attribute: {:?}", attrib);
                }
            }
        }
        for child in encryption_property.children() {
            todo!("Handle unknown child!");
        }
        Ok(enc_prop)
    }
}

impl TryFrom<EncryptionProperty> for XmlObject {
    type Error = SAMLError;

    fn try_from(encryption_prop: EncryptionProperty) -> Result<Self, Self::Error> {
        let mut xml_obj = XmlObject::new(
            Some(EncryptionProperty::NS_PREFIX.to_string()),
            EncryptionProperty::ELEMENT_NAME.to_string(),
            Some(EncryptionProperty::NS_URI.to_string()),
        );
        xml_obj.add_namespace(
            EncryptionProperty::NS_PREFIX.to_string(),
            EncryptionProperty::NS_URI.to_string(),
        );
        xml_obj.add_attribute(
            EncryptionProperty::ATTRIBUTE_TARGET.to_string(),
            encryption_prop.target.to_string(),
        );
        xml_obj.add_attribute(
            EncryptionProperty::ATTRIBUTE_ID.to_string(),
            encryption_prop.id.to_string(),
        );
        // TODO: Add unknown attributes
        for child in encryption_prop.unknown_children() {
            todo!("XmlObject::try_from<unknown child>");
        }
        Ok(xml_obj)
    }
}
