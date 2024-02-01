use std::cell::Ref;

use chrono::{DateTime, Utc};

use crate::{error::SAMLError, xml::XmlObject};

use super::parse_from_string;

#[derive(Clone, Debug, Default)]
pub struct SubjectConfirmationData {
    not_before: Option<DateTime<Utc>>,
    not_on_or_after: Option<DateTime<Utc>>,
    recipient: Option<String>,
    in_response_to: Option<String>,
    address: Option<String>,
}

/// implement getter and setter methods for SubjectConfirmationData
impl SubjectConfirmationData {
    const ATTRIB_NOT_BEFORE: &'static str = "NotBefore";
    const ATTRIB_NOT_ON_OR_AFTER: &'static str = "NotOnOrAfter";
    const ATTRIB_RECIPIENT: &'static str = "Recipient";
    const ATTRIB_IN_RESPONSE_TO: &'static str = "InResponseTo";
    const ATTRIB_ADDRESS: &'static str = "Address";

    pub fn not_before(&self) -> Option<&DateTime<Utc>> {
        self.not_before.as_ref()
    }

    pub fn set_not_before(&mut self, not_before: Option<DateTime<Utc>>) {
        self.not_before = not_before;
    }

    pub fn not_on_or_after(&self) -> Option<&DateTime<Utc>> {
        self.not_on_or_after.as_ref()
    }

    pub fn set_not_on_or_after(&mut self, not_on_or_after: Option<DateTime<Utc>>) {
        self.not_on_or_after = not_on_or_after;
    }

    pub fn recipient(&self) -> Option<&String> {
        self.recipient.as_ref()
    }

    pub fn set_recipient(&mut self, recipient: Option<String>) {
        self.recipient = recipient;
    }

    pub fn in_response_to(&self) -> Option<&String> {
        self.in_response_to.as_ref()
    }

    pub fn set_in_response_to(&mut self, in_response_to: Option<String>) {
        self.in_response_to = in_response_to;
    }

    pub fn address(&self) -> Option<&String> {
        self.address.as_ref()
    }

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
        Ok(subject_confirmation_data)
    }
}
