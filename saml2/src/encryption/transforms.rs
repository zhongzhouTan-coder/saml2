use std::cell::Ref;

use crate::{common::SAML2Obj, error::SAMLError, signature::transform::Transform, xml::XmlObject};

#[derive(Default, Debug)]
pub struct Transforms {
    transforms: Vec<Transform>,
}

impl SAML2Obj for Transforms {}

impl Transforms {
    const ELEMENT_TRANSFORM: &'static str = "Transform";

    const ELEMENT_NAME: &'static str = "Transforms";
    const NS_PREFIX: &'static str = "ds";
    const NS_URI: &'static str = "http://www.w3.org/2000/09/xmldsig#";

    #[inline]
    pub fn transforms(&self) -> &Vec<Transform> {
        &self.transforms
    }
}

impl TryFrom<Ref<'_, XmlObject>> for Transforms {
    type Error = SAMLError;

    fn try_from(xml_obj: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        let mut transforms = Transforms::default();
        for child in xml_obj.children() {
            let child = child.borrow();
            match child.q_name().local_name() {
                Transforms::ELEMENT_TRANSFORM => {
                    todo!("Transform::try_from<Ref<XmlObject>>")
                }
                _ => {}
            }
        }
        Ok(transforms)
    }
}

impl TryFrom<Transforms> for XmlObject {
    type Error = SAMLError;

    fn try_from(transforms: Transforms) -> Result<Self, Self::Error> {
        let mut xml_obj = XmlObject::new(
            Some(Transforms::NS_PREFIX.to_string()),
            Transforms::ELEMENT_NAME.to_string(),
            Some(Transforms::NS_URI.to_string()),
        );
        xml_obj.add_namespace(
            Transforms::NS_PREFIX.to_string(),
            Transforms::NS_URI.to_string(),
        );
        for transform in transforms.transforms() {
            todo!("XmlObject::try_from<Transform>")
        }
        Ok(xml_obj)
    }
}
