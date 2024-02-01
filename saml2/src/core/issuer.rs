use std::{cell::Ref, fmt::Display};

use crate::{error::SAMLError, xml::XmlObject};

use super::abstract_name_id_type::AbstractNameIDType;

#[derive(Clone, Debug)]
pub struct Issuer {
    name_qualifier: Option<String>,
    sp_name_qualifier: Option<String>,
    format: Option<String>,
    sp_provided_id: Option<String>,
    value: Option<String>,
}

impl Issuer {
    const ATTRIB_NAME_QUALIFIER: &'static str = "NameQualifier";
    const ATTRIB_SP_NAME_QUALIFIER: &'static str = "SPNameQualifier";
    const ATTRIB_FORMAT: &'static str = "Format";
    const ATTRIB_SP_PROVIDED_ID: &'static str = "SPProvidedID";

    pub fn new() -> Issuer {
        Issuer {
            name_qualifier: None,
            sp_name_qualifier: None,
            format: None,
            sp_provided_id: None,
            value: None,
        }
    }

    pub fn value(&self) -> Option<&String> {
        self.value.as_ref()
    }

    pub fn set_value(&mut self, value: Option<String>) {
        self.value = value
    }
}

impl Default for Issuer {
    fn default() -> Self {
        Self::new()
    }
}

impl AbstractNameIDType for Issuer {
    fn name_qualifier(&self) -> Option<&String> {
        self.name_qualifier.as_ref()
    }

    fn set_name_qualifier(&mut self, name_qualifier: Option<String>) {
        self.sp_name_qualifier = name_qualifier
    }

    fn sp_name_qualifier(&self) -> Option<&String> {
        self.sp_name_qualifier.as_ref()
    }

    fn set_sp_name_qualifier(&mut self, sp_name_qualifier: Option<String>) {
        self.sp_name_qualifier = sp_name_qualifier
    }

    fn format(&self) -> Option<&String> {
        self.format.as_ref()
    }

    fn set_format(&mut self, format: Option<String>) {
        self.format = format
    }

    fn sp_provided_id(&self) -> Option<&String> {
        self.sp_provided_id.as_ref()
    }

    fn set_sp_provided_id(&mut self, sp_provided_id: Option<String>) {
        self.sp_provided_id = sp_provided_id
    }
}

impl TryFrom<Ref<'_, XmlObject>> for Issuer {
    type Error = SAMLError;

    fn try_from(element: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        let mut issuer = Issuer::default();
        for attribute in element.attributes() {
            match attribute.0.as_str() {
                Issuer::ATTRIB_NAME_QUALIFIER => {
                    issuer.name_qualifier = Some(attribute.1.to_string());
                }
                Issuer::ATTRIB_SP_NAME_QUALIFIER => {
                    issuer.sp_name_qualifier = Some(attribute.1.to_string());
                }
                Issuer::ATTRIB_FORMAT => {
                    issuer.format = Some(attribute.1.to_string());
                }
                Issuer::ATTRIB_SP_PROVIDED_ID => {
                    issuer.sp_provided_id = Some(attribute.1.to_string());
                }
                _ => {}
            }
        }
        if let Some(value) = element.text() {
            issuer.value = Some(value.to_string());
        }
        Ok(issuer)
    }
}

impl Display for Issuer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Issuer: name_qualifier={:?}, sp_name_qualifier={:?}, format={:?}, sp_provided_id={:?}",
            self.name_qualifier, self.sp_name_qualifier, self.format, self.sp_provided_id
        );
        Ok(())
    }
}
