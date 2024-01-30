use std::cell::Ref;

use crate::{error::SAMLError, xml::XmlObject};

#[derive(Clone, Debug)]
pub struct NameIDPolicy {
    format: Option<String>,
    sp_name_qualifier: Option<String>,
    allows_create: Option<String>,
}

impl NameIDPolicy {
    const ATTRIB_ALLOW_CREATE: &'static str = "AllowCreate";
    const ATTRIB_FORMAT: &'static str = "Format";
    const ATTRIB_SP_NAME_QUALIFIER: &'static str = "SPNameQualifier";

    pub fn new() -> Self {
        Self {
            format: None,
            sp_name_qualifier: None,
            allows_create: None,
        }
    }

    pub fn format(&self) -> Option<&String> {
        self.format.as_ref()
    }

    pub fn set_format(&mut self, format: Option<String>) {
        self.format = format
    }

    pub fn sp_name_qualifier(&self) -> Option<&String> {
        self.sp_name_qualifier.as_ref()
    }

    pub fn set_sp_name_qualifier(&mut self, sp_name_qualifier: Option<String>) {
        self.sp_name_qualifier = sp_name_qualifier
    }

    pub fn allows_create(&self) -> Option<&String> {
        self.allows_create.as_ref()
    }

    pub fn set_allows_create(&mut self, allows_create: Option<String>) {
        self.allows_create = allows_create
    }
}

impl TryFrom<Ref<'_, XmlObject>> for NameIDPolicy {
    type Error = SAMLError;

    fn try_from(element: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        let mut name_id_policy = NameIDPolicy::new();
        for attribute in element.attributes() {
            match attribute.0.as_str() {
                NameIDPolicy::ATTRIB_FORMAT => {
                    name_id_policy.format = Some(attribute.1.to_string());
                }
                NameIDPolicy::ATTRIB_SP_NAME_QUALIFIER => {
                    name_id_policy.sp_name_qualifier = Some(attribute.1.to_string());
                }
                NameIDPolicy::ATTRIB_ALLOW_CREATE => {
                    name_id_policy.allows_create = Some(attribute.1.to_string());
                }
                _ => {}
            }
        }
        Ok(name_id_policy)
    }
}
