use std::cell::Ref;

use crate::{common::SAML2Obj, error::SAMLError, xml::XmlObject};

#[derive(Default, Debug)]
pub struct SubjectLocality {
    address: Option<String>,
    dns_name: Option<String>,
}

impl SAML2Obj for SubjectLocality {}

impl SubjectLocality {
    const ATTRIB_ADDRESS: &'static str = "Address";
    const ATTRIB_DNS_NAME: &'static str = "DNSName";

    const ELEMENT_NAME: &'static str = "SubjectLocality";
    const NS_PREFIX: &'static str = "saml";
    const NS_URI: &'static str = "urn:oasis:names:tc:SAML:2.0:assertion";

    #[inline]
    pub fn address(&self) -> Option<&String> {
        self.address.as_ref()
    }

    #[inline]
    pub fn set_address(&mut self, address: Option<String>) {
        self.address = address;
    }

    #[inline]
    pub fn dns_name(&self) -> Option<&String> {
        self.dns_name.as_ref()
    }

    #[inline]
    pub fn set_dns_name(&mut self, dns_name: Option<String>) {
        self.dns_name = dns_name;
    }
}

impl TryFrom<Ref<'_, XmlObject>> for SubjectLocality {
    type Error = SAMLError;

    fn try_from(object: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        let mut subject_locality = SubjectLocality::default();
        for attrib in object.attributes() {
            match attrib.0.as_str() {
                SubjectLocality::ATTRIB_ADDRESS => {
                    subject_locality.set_address(Some(attrib.1.to_string()));
                }
                SubjectLocality::ATTRIB_DNS_NAME => {
                    subject_locality.set_dns_name(Some(attrib.1.to_string()));
                }
                _ => {}
            }
        }
        Ok(subject_locality)
    }
}

impl TryFrom<SubjectLocality> for XmlObject {
    type Error = SAMLError;

    fn try_from(subject_locality: SubjectLocality) -> Result<Self, Self::Error> {
        let mut xml_object = XmlObject::new(
            Some(SubjectLocality::NS_PREFIX.to_string()),
            SubjectLocality::ELEMENT_NAME.to_string(),
            Some(SubjectLocality::NS_URI.to_string()),
        );
        if let Some(address) = subject_locality.address() {
            xml_object.add_attribute(
                SubjectLocality::ATTRIB_ADDRESS.to_string(),
                address.to_string(),
            );
        }
        if let Some(dns_name) = subject_locality.dns_name() {
            xml_object.add_attribute(
                SubjectLocality::ATTRIB_DNS_NAME.to_string(),
                dns_name.to_string(),
            );
        }
        Ok(xml_object)
    }
}
