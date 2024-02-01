use std::cell::Ref;

use crate::{error::SAMLError, xml::XmlObject};

#[derive(Default, Debug)]
pub struct StatusCode {
    value: String,
    status_code: Option<Box<StatusCode>>,
}

/// implement getter and setter for status code
impl StatusCode {
    pub fn value(&self) -> &String {
        &self.value
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }

    pub fn status_code(&self) -> Option<&Box<StatusCode>> {
        self.status_code.as_ref()
    }

    pub fn set_status_code(&mut self, status_code: Option<Box<StatusCode>>) {
        self.status_code = status_code;
    }
}

/// impl TryFrom<Ref<'_, XmlObject>> for StatusCode
impl TryFrom<Ref<'_, XmlObject>> for StatusCode {
    type Error = SAMLError;

    fn try_from(object: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        let mut status_code = StatusCode::default();
        for attribute in object.attributes() {
            match attribute.0.as_str() {
                "Value" => {
                    status_code.set_value(attribute.1.to_string());
                }
                _ => {}
            }
        }
        Ok(status_code)
    }
}
