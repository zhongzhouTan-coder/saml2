use std::cell::Ref;

use crate::{error::SAMLError, xml::XmlObject};

use super::{
    base_id::BaseID, encrypted_id::EncryptedID, name_id::NameID,
    subject_confirmation_data::SubjectConfirmationData,
};

#[derive(Clone, Debug, Default)]
pub struct SubjectConfirmation {
    base_id: Option<BaseID>,
    name_id: Option<NameID>,
    encrypted_id: Option<EncryptedID>,
    subject_confirmation_data: Option<SubjectConfirmationData>,
    method: String,
}

/// implement getter and setter methods for SubjectConfirmation
impl SubjectConfirmation {
    const ATTRIB_METHOD: &'static str = "Method";

    const CHILD_BASE_ID: &'static str = "BaseID";
    const CHILD_NAME_ID: &'static str = "NameID";
    const CHILD_ENCRYPTED_ID: &'static str = "EncryptedID";
    const CHILD_SUBJECT_CONFIRMATION_DATA: &'static str = "SubjectConfirmationData";

    pub fn new() -> Self {
        Self::default()
    }

    /// getter
    pub fn base_id(&self) -> Option<&BaseID> {
        self.base_id.as_ref()
    }

    pub fn set_base_id(&mut self, base_id: Option<BaseID>) {
        self.base_id = base_id;
    }

    pub fn name_id(&self) -> Option<&NameID> {
        self.name_id.as_ref()
    }

    pub fn set_name_id(&mut self, name_id: Option<NameID>) {
        self.name_id = name_id;
    }

    pub fn encrypted_id(&self) -> Option<&EncryptedID> {
        self.encrypted_id.as_ref()
    }

    pub fn set_encrypted_id(&mut self, encrypted_id: Option<EncryptedID>) {
        self.encrypted_id = encrypted_id;
    }

    pub fn subject_confirmation_data(&self) -> Option<&SubjectConfirmationData> {
        self.subject_confirmation_data.as_ref()
    }

    pub fn set_subject_confirmation_data(
        &mut self,
        subject_confirmation_data: Option<SubjectConfirmationData>,
    ) {
        self.subject_confirmation_data = subject_confirmation_data;
    }

    pub fn method(&self) -> &str {
        &self.method
    }

    pub fn set_method(&mut self, method: String) {
        self.method = method;
    }
}

impl TryFrom<Ref<'_, XmlObject>> for SubjectConfirmation {
    type Error = SAMLError;

    fn try_from(element: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        let mut subject_confirmation = SubjectConfirmation::default();
        for attribute in element.attributes() {
            match attribute.0.as_str() {
                Self::ATTRIB_METHOD => {
                    subject_confirmation.set_method(attribute.1.to_string());
                }
                _ => {
                    println!("subject confirmation attribute: {}", attribute.0);
                }
            }
        }

        for child in element.children() {
            let child = child.borrow();
            match child.q_name().local_name() {
                Self::CHILD_BASE_ID => {
                    subject_confirmation.set_base_id(Some(BaseID::try_from(child)?));
                }
                Self::CHILD_NAME_ID => {
                    subject_confirmation.set_name_id(Some(NameID::try_from(child)?));
                }
                Self::CHILD_ENCRYPTED_ID => {
                    // todo
                }
                Self::CHILD_SUBJECT_CONFIRMATION_DATA => {
                    subject_confirmation.set_subject_confirmation_data(Some(
                        SubjectConfirmationData::try_from(child)?,
                    ));
                }
                _ => {
                    println!(
                        "subject confirmation child: {:?}",
                        child.q_name().local_name()
                    );
                }
            }
        }

        Ok(subject_confirmation)
    }
}
