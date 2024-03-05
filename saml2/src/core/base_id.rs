use std::cell::Ref;

use crate::{common::SAML2Obj, error::SAMLError, xml::XmlObject};

#[derive(Clone, Debug)]
pub struct BaseID {
    name_qualifier: Option<String>,
    sp_name_qualifier: Option<String>,
}

impl SAML2Obj for BaseID {}

impl BaseID {
    const ATTRIB_NAME_QUALIFIER: &'static str = "NameQualifier";
    const ATTRIB_SP_NAME_QUALIFIER: &'static str = "SPNameQualifier";

    const ELEMENT_NAME: &'static str = "BaseID";
    const NS_PREFIX: &'static str = "saml2";
    const NS_URI: &'static str = "urn:oasis:names:tc:SAML:2.0:assertion";

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

impl TryFrom<BaseID> for XmlObject {
    type Error = SAMLError;

    fn try_from(value: BaseID) -> Result<Self, Self::Error> {
        let mut object = XmlObject::new(
            Some(BaseID::NS_URI.to_string()),
            BaseID::ELEMENT_NAME.to_string(),
            Some(BaseID::NS_PREFIX.to_string()),
        );
        if let Some(name_qualifier) = value.name_qualifier() {
            object.add_attribute(
                BaseID::ATTRIB_NAME_QUALIFIER.to_string(),
                name_qualifier.to_string(),
            );
        }
        if let Some(sp_name_qualifier) = value.sp_name_qualifier() {
            object.add_attribute(
                BaseID::ATTRIB_SP_NAME_QUALIFIER.to_string(),
                sp_name_qualifier.to_string(),
            );
        }
        Ok(object)
    }
}
