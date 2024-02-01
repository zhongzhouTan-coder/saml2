use std::cell::Ref;

use crate::{core::status_code::StatusCode, error::SAMLError, xml::XmlObject};

#[derive(Default, Debug)]
pub struct Status {
    status_code: StatusCode,
    status_message: Option<String>,
    status_detail: Option<String>,
}

/// implement getter and setter for status
impl Status {
    pub fn status_code(&self) -> &StatusCode {
        &self.status_code
    }

    pub fn set_status_code(&mut self, status_code: StatusCode) {
        self.status_code = status_code;
    }

    pub fn status_message(&self) -> Option<&String> {
        self.status_message.as_ref()
    }

    pub fn set_status_message(&mut self, status_message: Option<String>) {
        self.status_message = status_message;
    }

    pub fn status_detail(&self) -> Option<&String> {
        self.status_detail.as_ref()
    }

    pub fn set_status_detail(&mut self, status_detail: Option<String>) {
        self.status_detail = status_detail;
    }
}

/// impl TryFrom<Ref<'_, XmlObject>> for Status
impl TryFrom<Ref<'_, XmlObject>> for Status {
    type Error = SAMLError;

    fn try_from(object: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        let mut status = Status::default();
        for attribute in object.attributes() {
            match attribute.0.as_str() {
                "StatusMessage" => {
                    status.set_status_message(Some(attribute.1.to_string()));
                }
                "StatusDetail" => {
                    status.set_status_detail(Some(attribute.1.to_string()));
                }
                _ => {}
            }
        }
        for child in object.children() {
            let child = child.borrow();
            match child.q_name().local_name() {
                "StatusCode" => {
                    status.set_status_code(StatusCode::try_from(child)?);
                }
                _ => {}
            }
        }
        Ok(status)
    }
}
