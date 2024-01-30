use std::cell::Ref;

use crate::{error::SAMLError, xml::XmlObject};

use super::{
    base_id::BaseID, encrypted_id::EncryptedID, name_id::NameID,
    subject_confirmation::SubjectConfirmation,
};

#[derive(Clone, Default, Debug)]
pub struct Subject {
    base_id: Option<BaseID>,
    name_id: Option<NameID>,
    encrypted_id: Option<EncryptedID>,
    subject_confirmations: Vec<SubjectConfirmation>,
}

impl Subject {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn base_id(&self) -> Option<&BaseID> {
        self.base_id.as_ref()
    }

    pub fn name_id(&self) -> Option<&NameID> {
        self.name_id.as_ref()
    }

    pub fn encrypted_id(&self) -> Option<&EncryptedID> {
        self.encrypted_id.as_ref()
    }

    pub fn subject_confirmations(&self) -> &[SubjectConfirmation] {
        &self.subject_confirmations
    }

    pub fn set_base_id(&mut self, base_id: Option<BaseID>) {
        self.base_id = base_id;
    }

    pub fn set_name_id(&mut self, name_id: Option<NameID>) {
        self.name_id = name_id;
    }

    pub fn set_encrypted_id(&mut self, encrypted_id: Option<EncryptedID>) {
        self.encrypted_id = encrypted_id;
    }

    pub fn set_subject_confirmations(&mut self, subject_confirmations: Vec<SubjectConfirmation>) {
        self.subject_confirmations = subject_confirmations;
    }
}

impl TryFrom<Ref<'_, XmlObject>> for Subject {
    type Error = SAMLError;

    fn try_from(element: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        let mut subject = Subject::new();
        for child in element.children() {
            let child = child.borrow();
            match child.q_name().local_name() {
                _ => {
                    println!("subject child: {:?}", child.q_name().local_name());
                }
            }
        }
        Ok(subject)
    }
}
