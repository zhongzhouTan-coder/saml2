use std::cell::Ref;

use crate::{error::SAMLError, xml::XmlObject};

#[derive(Debug, Default)]
pub struct Audience {
    value: String,
}

impl Audience {
    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }
}

/// implement tryFrom Ref<'_, XmlObject> for Audience

impl TryFrom<Ref<'_, XmlObject>> for Audience {
    type Error = SAMLError;

    fn try_from(object: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        match object.text() {
            Some(value) => Ok(Audience {
                value: value.to_string(),
            }),
            None => Err(SAMLError::UnmarshallingError("Invalid XML".to_string())),
        }
    }
}
