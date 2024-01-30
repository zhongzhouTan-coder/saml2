use std::cell::Ref;

use crate::{error::SAMLError, xml::XmlObject};

#[derive(Clone, Debug)]
pub struct Extensions {}

impl TryFrom<Ref<'_, XmlObject>> for Extensions {
    type Error = SAMLError;

    fn try_from(element: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        Ok(Extensions {})
    }
}
