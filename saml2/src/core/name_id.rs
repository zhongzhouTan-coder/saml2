use std::cell::Ref;

use crate::{error::SAMLError, xml::XmlObject};

use super::abstract_name_id_type::AbstractNameIDType;

#[derive(Clone, Debug, Default)]
pub struct NameID {
    name_qualifier: Option<String>,
    sp_name_qualifier: Option<String>,
    format: Option<String>,
    sp_provided_id: Option<String>,
}

impl NameID {
    const ATTRIB_NAME_QUALIFIER: &'static str = "NameQualifier";
    const ATTRIB_SP_NAME_QUALIFIER: &'static str = "SPNameQualifier";
    const ATTRIB_FORMAT: &'static str = "Format";
    const ATTRIB_SP_PROVIDED_ID: &'static str = "SPProvidedID";
}

impl AbstractNameIDType for NameID {
    fn name_qualifier(&self) -> Option<&String> {
        self.name_qualifier.as_ref()
    }

    fn set_name_qualifier(&mut self, name_qualifier: Option<String>) {
        self.name_qualifier = name_qualifier;
    }

    fn sp_name_qualifier(&self) -> Option<&String> {
        self.sp_name_qualifier.as_ref()
    }

    fn set_sp_name_qualifier(&mut self, sp_name_qualifier: Option<String>) {
        self.sp_name_qualifier = sp_name_qualifier;
    }

    fn format(&self) -> Option<&String> {
        self.format.as_ref()
    }

    fn set_format(&mut self, format: Option<String>) {
        self.format = format;
    }

    fn sp_provided_id(&self) -> Option<&String> {
        self.sp_provided_id.as_ref()
    }

    fn set_sp_provided_id(&mut self, sp_provided_id: Option<String>) {
        self.sp_provided_id = sp_provided_id;
    }
}

/// implement tryFrom Ref<'_, XmlObject> for NameID

impl TryFrom<Ref<'_, XmlObject>> for NameID {
    type Error = SAMLError;

    fn try_from(element: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        let mut name_id = NameID::default();
        for attribute in element.attributes() {
            match attribute.0.as_str() {
                Self::ATTRIB_NAME_QUALIFIER => {
                    name_id.set_name_qualifier(Some(attribute.1.to_string()));
                }
                Self::ATTRIB_SP_NAME_QUALIFIER => {
                    name_id.set_sp_name_qualifier(Some(attribute.1.to_string()));
                }
                Self::ATTRIB_FORMAT => {
                    name_id.set_format(Some(attribute.1.to_string()));
                }
                Self::ATTRIB_SP_PROVIDED_ID => {
                    name_id.set_sp_provided_id(Some(attribute.1.to_string()));
                }
                _ => {}
            }
        }
        Ok(name_id)
    }
}
