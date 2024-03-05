use std::cell::Ref;

use crate::{common::SAML2Obj, error::SAMLError, xml::XmlObject};

use super::abstract_name_id_type::AbstractNameIDType;

#[derive(Clone, Debug, Default)]
pub struct NameID {
    name_qualifier: Option<String>,
    sp_name_qualifier: Option<String>,
    format: Option<String>,
    sp_provided_id: Option<String>,
}

impl SAML2Obj for NameID {}

impl NameID {
    const ATTRIB_NAME_QUALIFIER: &'static str = "NameQualifier";
    const ATTRIB_SP_NAME_QUALIFIER: &'static str = "SPNameQualifier";
    const ATTRIB_FORMAT: &'static str = "Format";
    const ATTRIB_SP_PROVIDED_ID: &'static str = "SPProvidedID";

    const ELEMENT_NAME: &'static str = "NameID";
    const NS_PREFIX: &'static str = "saml2";
    const NS_URI: &'static str = "urn:oasis:names:tc:SAML:2.0:assertion";
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

impl TryFrom<Ref<'_, XmlObject>> for NameID {
    type Error = SAMLError;

    fn try_from(element: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        let mut name_id = NameID::default();
        for attribute in element.attributes() {
            match attribute.0.as_str() {
                NameID::ATTRIB_NAME_QUALIFIER => {
                    name_id.set_name_qualifier(Some(attribute.1.to_string()));
                }
                NameID::ATTRIB_SP_NAME_QUALIFIER => {
                    name_id.set_sp_name_qualifier(Some(attribute.1.to_string()));
                }
                NameID::ATTRIB_FORMAT => {
                    name_id.set_format(Some(attribute.1.to_string()));
                }
                NameID::ATTRIB_SP_PROVIDED_ID => {
                    name_id.set_sp_provided_id(Some(attribute.1.to_string()));
                }
                _ => {}
            }
        }
        Ok(name_id)
    }
}

impl TryFrom<NameID> for XmlObject {
    type Error = SAMLError;

    fn try_from(value: NameID) -> Result<Self, Self::Error> {
        let mut xml_object = XmlObject::new(
            Some(NameID::NS_URI.to_string()),
            NameID::ELEMENT_NAME.to_string(),
            Some(NameID::NS_PREFIX.to_string()),
        );
        xml_object.add_namespace(NameID::NS_PREFIX.to_string(), NameID::NS_URI.to_string());
        if let Some(name_qualifier) = value.name_qualifier() {
            xml_object.add_attribute(
                NameID::ATTRIB_NAME_QUALIFIER.to_string(),
                name_qualifier.to_string(),
            );
        }
        if let Some(sp_name_qualifier) = value.sp_name_qualifier() {
            xml_object.add_attribute(
                NameID::ATTRIB_SP_NAME_QUALIFIER.to_string(),
                sp_name_qualifier.to_string(),
            );
        }
        if let Some(format) = value.format() {
            xml_object.add_attribute(NameID::ATTRIB_FORMAT.to_string(), format.to_string());
        }
        if let Some(sp_provided_id) = value.sp_provided_id() {
            xml_object.add_attribute(
                NameID::ATTRIB_SP_PROVIDED_ID.to_string(),
                sp_provided_id.to_string(),
            );
        }
        Ok(xml_object)
    }
}
