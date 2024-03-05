use std::cell::Ref;

use chrono::{DateTime, Utc};

use crate::{common::SAML2Obj, error::SAMLError, util::AttributeMap, xml::XmlObject};

use super::parse_from_string;

#[derive(Debug, Default)]
pub struct SubjectConfirmationData {
    not_before: Option<DateTime<Utc>>,
    not_on_or_after: Option<DateTime<Utc>>,
    recipient: Option<String>,
    in_response_to: Option<String>,
    address: Option<String>,
    unknown_attributes: AttributeMap,
    unknown_children: Vec<Box<dyn SAML2Obj>>,
}

impl SAML2Obj for SubjectConfirmationData {}

impl SubjectConfirmationData {
    const ATTRIB_NOT_BEFORE: &'static str = "NotBefore";
    const ATTRIB_NOT_ON_OR_AFTER: &'static str = "NotOnOrAfter";
    const ATTRIB_RECIPIENT: &'static str = "Recipient";
    const ATTRIB_IN_RESPONSE_TO: &'static str = "InResponseTo";
    const ATTRIB_ADDRESS: &'static str = "Address";

    const ELEMENT_NAME: &'static str = "SubjectConfirmationData";
    const NS_PREFIX: &'static str = "saml2";
    const NS_URI: &'static str = "urn:oasis:names:tc:SAML:2.0:assertion";

    #[inline]
    pub fn not_before(&self) -> Option<&DateTime<Utc>> {
        self.not_before.as_ref()
    }

    #[inline]
    pub fn set_not_before(&mut self, not_before: Option<DateTime<Utc>>) {
        self.not_before = not_before;
    }

    #[inline]
    pub fn not_on_or_after(&self) -> Option<&DateTime<Utc>> {
        self.not_on_or_after.as_ref()
    }

    #[inline]
    pub fn set_not_on_or_after(&mut self, not_on_or_after: Option<DateTime<Utc>>) {
        self.not_on_or_after = not_on_or_after;
    }

    #[inline]
    pub fn recipient(&self) -> Option<&String> {
        self.recipient.as_ref()
    }

    #[inline]
    pub fn set_recipient(&mut self, recipient: Option<String>) {
        self.recipient = recipient;
    }

    #[inline]
    pub fn in_response_to(&self) -> Option<&String> {
        self.in_response_to.as_ref()
    }

    #[inline]
    pub fn set_in_response_to(&mut self, in_response_to: Option<String>) {
        self.in_response_to = in_response_to;
    }

    #[inline]
    pub fn address(&self) -> Option<&String> {
        self.address.as_ref()
    }

    #[inline]
    pub fn set_address(&mut self, address: Option<String>) {
        self.address = address;
    }
}

impl TryFrom<Ref<'_, XmlObject>> for SubjectConfirmationData {
    type Error = SAMLError;

    fn try_from(element: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        let mut subject_confirmation_data = SubjectConfirmationData::default();
        for attribute in element.attributes() {
            let value = attribute.1.as_str();
            match attribute.0.as_str() {
                Self::ATTRIB_NOT_BEFORE => {
                    subject_confirmation_data.set_not_before(Some(parse_from_string(value)?));
                }
                Self::ATTRIB_NOT_ON_OR_AFTER => {
                    subject_confirmation_data.set_not_on_or_after(Some(parse_from_string(value)?));
                }
                Self::ATTRIB_RECIPIENT => {
                    subject_confirmation_data.set_recipient(Some(attribute.1.to_string()));
                }
                Self::ATTRIB_IN_RESPONSE_TO => {
                    subject_confirmation_data.set_in_response_to(Some(attribute.1.to_string()));
                }
                Self::ATTRIB_ADDRESS => {
                    subject_confirmation_data.set_address(Some(attribute.1.to_string()));
                }
                _ => {
                    println!("subject confirmation data attribute: {}", attribute.0);
                }
            }
        }
        for child in element.children() {
            let child = child.borrow();
            let name = child.q_name().local_name();
            println!("subject confirmation data child: {}", name);
        }
        Ok(subject_confirmation_data)
    }
}

impl TryFrom<SubjectConfirmationData> for XmlObject {
    type Error = SAMLError;

    fn try_from(subject_confirmation_data: SubjectConfirmationData) -> Result<Self, Self::Error> {
        let mut xml_object = XmlObject::new(
            Some(SubjectConfirmationData::NS_PREFIX.to_string()),
            SubjectConfirmationData::ELEMENT_NAME.to_string(),
            Some(SubjectConfirmationData::NS_URI.to_string()),
        );
        xml_object.add_namespace(
            SubjectConfirmationData::NS_PREFIX.to_string(),
            SubjectConfirmationData::NS_URI.to_string(),
        );
        if let Some(not_before) = subject_confirmation_data.not_before() {
            xml_object.add_attribute(
                SubjectConfirmationData::ATTRIB_NOT_BEFORE.to_string(),
                not_before.to_rfc3339(),
            );
        }
        if let Some(not_on_or_after) = subject_confirmation_data.not_on_or_after() {
            xml_object.add_attribute(
                SubjectConfirmationData::ATTRIB_NOT_ON_OR_AFTER.to_string(),
                not_on_or_after.to_rfc3339(),
            );
        }
        if let Some(recipient) = subject_confirmation_data.recipient() {
            xml_object.add_attribute(
                SubjectConfirmationData::ATTRIB_RECIPIENT.to_string(),
                recipient.to_string(),
            );
        }
        if let Some(in_response_to) = subject_confirmation_data.in_response_to() {
            xml_object.add_attribute(
                SubjectConfirmationData::ATTRIB_IN_RESPONSE_TO.to_string(),
                in_response_to.to_string(),
            );
        }
        if let Some(address) = subject_confirmation_data.address() {
            xml_object.add_attribute(
                SubjectConfirmationData::ATTRIB_ADDRESS.to_string(),
                address.to_string(),
            );
        }
        // TODO: add unknown attributes and children
        Ok(xml_object)
    }
}
