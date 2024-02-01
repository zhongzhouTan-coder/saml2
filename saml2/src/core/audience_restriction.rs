use std::cell::Ref;

use crate::{error::SAMLError, xml::XmlObject};

use super::audience::Audience;

#[derive(Debug, Default)]
pub struct AudienceRestriction {
    audiences: Vec<Audience>,
}

impl AudienceRestriction {
    const CHILD_AUDIENCE: &'static str = "Audience";

    pub fn audiences(&self) -> &Vec<Audience> {
        &self.audiences
    }

    pub fn add_audiences(&mut self, audience: Audience) {
        self.audiences.push(audience)
    }
}

/// implement tryFrom Ref<'_, XmlObject> for AudienceRestriction
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
