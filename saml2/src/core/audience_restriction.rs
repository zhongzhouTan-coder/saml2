use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use crate::{common::SAML2Obj, error::SAMLError, xml::XmlObject};

use super::audience::Audience;

#[derive(Debug, Default)]
pub struct AudienceRestriction {
    audiences: Vec<Audience>,
}

impl SAML2Obj for AudienceRestriction {}

impl AudienceRestriction {
    const CHILD_AUDIENCE: &'static str = "Audience";

    const ELEMENT_NAME: &'static str = "AudienceRestriction";
    const NS_PREFIX: &'static str = "saml2";
    const NS_URI: &'static str = "urn:oasis:names:tc:SAML:2.0:assertion";

    #[inline]
    pub fn audiences(&self) -> &Vec<Audience> {
        &self.audiences
    }

    #[inline]
    pub fn add_audiences(&mut self, audience: Audience) {
        self.audiences.push(audience)
    }
}

impl TryFrom<Ref<'_, XmlObject>> for AudienceRestriction {
    type Error = SAMLError;

    fn try_from(object: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        let mut audience_restriction = AudienceRestriction::default();
        for child in object.children() {
            let child = child.borrow();
            match child.q_name().local_name() {
                Self::CHILD_AUDIENCE => {
                    audience_restriction.add_audiences(Audience::try_from(child)?);
                }
                _ => {}
            }
        }
        Ok(audience_restriction)
    }
}

impl TryFrom<AudienceRestriction> for XmlObject {
    type Error = SAMLError;

    fn try_from(audience_restriction: AudienceRestriction) -> Result<Self, Self::Error> {
        let mut xml_obj = XmlObject::new(
            Some(AudienceRestriction::NS_PREFIX.to_string()),
            AudienceRestriction::ELEMENT_NAME.to_string(),
            Some(AudienceRestriction::NS_URI.to_string()),
        );
        xml_obj.add_namespace(
            AudienceRestriction::NS_PREFIX.to_string(),
            AudienceRestriction::NS_URI.to_string(),
        );
        for audience in audience_restriction.audiences {
            xml_obj.add_child(Rc::new(RefCell::new(XmlObject::try_from(audience)?)));
        }
        Ok(xml_obj)
    }
}
