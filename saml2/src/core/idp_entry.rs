use std::cell::Ref;

use crate::{common::SAML2Obj, error::SAMLError, xml::XmlObject};

#[derive(Default, Debug)]
pub struct IDPEntry {
    provider_id: String,
    name: Option<String>,
    loc: Option<String>,
}

impl SAML2Obj for IDPEntry {}

impl IDPEntry {
    const ATTRIB_PROVIDER_ID: &'static str = "ProviderID";
    const ATTRIB_NAME: &'static str = "Name";
    const ATTRIB_LOC: &'static str = "Loc";

    const ELEMENT_NAME: &'static str = "IDPEntry";
    const NS_PREFIX: &'static str = "saml2p";
    const NS_URI: &'static str = "urn:oasis:names:tc:SAML:2.0:protocol";

    #[inline]
    pub fn provider_id(&self) -> &str {
        &self.provider_id
    }

    #[inline]
    pub fn set_provider_id(&mut self, provider_id: String) {
        self.provider_id = provider_id;
    }

    #[inline]
    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    #[inline]
    pub fn set_name(&mut self, name: Option<String>) {
        self.name = name;
    }

    #[inline]
    pub fn loc(&self) -> Option<&String> {
        self.loc.as_ref()
    }

    #[inline]
    pub fn set_loc(&mut self, loc: Option<String>) {
        self.loc = loc;
    }
}

impl TryFrom<Ref<'_, XmlObject>> for IDPEntry {
    type Error = SAMLError;

    fn try_from(element: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        let mut idp_entry = IDPEntry::default();
        for attrib in element.attributes() {
            match attrib.0.as_str() {
                IDPEntry::ATTRIB_PROVIDER_ID => {
                    idp_entry.set_provider_id(attrib.1.clone());
                }
                IDPEntry::ATTRIB_NAME => {
                    idp_entry.set_name(Some(attrib.1.clone()));
                }
                IDPEntry::ATTRIB_LOC => {
                    idp_entry.set_loc(Some(attrib.1.clone()));
                }
                _ => {}
            }
        }
        Ok(idp_entry)
    }
}

impl TryFrom<IDPEntry> for XmlObject {
    type Error = SAMLError;

    fn try_from(idp_entry: IDPEntry) -> Result<Self, Self::Error> {
        let mut xml_object = XmlObject::new(
            Some(IDPEntry::NS_PREFIX.to_string()),
            IDPEntry::ELEMENT_NAME.to_string(),
            Some(IDPEntry::NS_URI.to_string()),
        );
        xml_object.add_attribute(
            IDPEntry::ATTRIB_PROVIDER_ID.to_string(),
            idp_entry.provider_id,
        );
        if let Some(name) = idp_entry.name {
            xml_object.add_attribute(IDPEntry::ATTRIB_NAME.to_string(), name);
        }
        if let Some(loc) = idp_entry.loc {
            xml_object.add_attribute(IDPEntry::ATTRIB_LOC.to_string(), loc);
        }
        Ok(xml_object)
    }
}
