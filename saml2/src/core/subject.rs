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
    value: Option<String>,
}

impl Subject {
    const CHILD_BASE_ID: &'static str = "BaseID";
    const CHILD_NAME_ID: &'static str = "NameID";
    const CHILD_ENCRYPTED_ID: &'static str = "EncryptedID";
    const CHILD_SUBJECT_CONFIRMATION: &'static str = "SubjectConfirmation";

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

    pub fn value(&self) -> Option<&String> {
        self.value.as_ref()
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

    pub fn add_subject_confirmation(&mut self, subject_confirmation: SubjectConfirmation) {
        self.subject_confirmations.push(subject_confirmation);
    }

    pub fn set_value(&mut self, value: Option<String>) {
        self.value = value;
    }
}

impl TryFrom<Ref<'_, XmlObject>> for Subject {
    type Error = SAMLError;

    fn try_from(element: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        let mut subject = Subject::new();
        for child in element.children() {
            let child = child.borrow();
            match child.q_name().local_name() {
                Self::CHILD_BASE_ID => {
                    subject.set_base_id(Some(BaseID::try_from(child)?));
                }
                Self::CHILD_NAME_ID => {
                    subject.set_name_id(Some(NameID::try_from(child)?));
                }
                Self::CHILD_ENCRYPTED_ID => {
                    // todo
                }
                Self::CHILD_SUBJECT_CONFIRMATION => {
                    subject.add_subject_confirmation(SubjectConfirmation::try_from(child)?);
                }
                _ => {
                    println!("subject child: {:?}", child.q_name().local_name());
                }
            }
        }
        if let Some(value) = element.text() {
            subject.set_value(Some(value.to_string()));
        }
        Ok(subject)
    }
}
