use std::cell::Ref;

use crate::{error::SAMLError, xml::XmlObject};

#[derive(Clone, Debug)]
pub struct BaseID {
    name_qualifier: Option<String>,
    sp_name_qualifier: Option<String>,
}

impl BaseID {
    const ATTRIB_NAME_QUALIFIER: &'static str = "NameQualifier";
    const ATTRIB_SP_NAME_QUALIFIER: &'static str = "SPNameQualifier";

    pub fn new(name_qualifier: Option<String>, sp_name_qualifier: Option<String>) -> Self {
        BaseID {
            name_qualifier,
            sp_name_qualifier,
        }
    }

    pub fn name_qualifier(&self) -> Option<&String> {
        self.name_qualifier.as_ref()
    }

    pub fn set_name_qualifier(&mut self, name_qualifier: Option<String>) {
        self.name_qualifier = name_qualifier;
    }

    pub fn sp_name_qualifier(&self) -> Option<&String> {
        self.sp_name_qualifier.as_ref()
    }

    pub fn set_sp_name_qualifier(&mut self, sp_name_qualifier: Option<String>) {
        self.sp_name_qualifier = sp_name_qualifier;
    }
}

/// implement tryFrom Ref<'_, XmlObject> for BaseID

impl TryFrom<Ref<'_, XmlObject>> for BaseID {
    type Error = SAMLError;

    fn try_from(element: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        let mut base_id = BaseID::new(None, None);
        for attribute in element.attributes() {
            match attribute.0.as_str() {
                Self::ATTRIB_NAME_QUALIFIER => {
                    base_id.set_name_qualifier(Some(attribute.1.to_string()));
                }
                Self::ATTRIB_SP_NAME_QUALIFIER => {
                    base_id.set_sp_name_qualifier(Some(attribute.1.to_string()));
                }
                _ => {}
            }
        }
        Ok(base_id)
    }
}
