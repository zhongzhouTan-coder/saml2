use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use crate::{common::SAML2Obj, error::SAMLError, xml::XmlObject};

use super::{
    base_id::BaseID, encrypted_id::EncryptedID, name_id::NameID,
    subject_confirmation::SubjectConfirmation,
};

#[derive(Default, Debug)]
pub struct Subject {
    base_id: Option<BaseID>,
    name_id: Option<NameID>,
    encrypted_id: Option<EncryptedID>,
    subject_confirmations: Vec<SubjectConfirmation>,
}

impl SAML2Obj for Subject {}

impl Subject {
    const CHILD_BASE_ID: &'static str = "BaseID";
    const CHILD_NAME_ID: &'static str = "NameID";
    const CHILD_ENCRYPTED_ID: &'static str = "EncryptedID";
    const CHILD_SUBJECT_CONFIRMATION: &'static str = "SubjectConfirmation";

    const ELEMENT_NAME: &'static str = "Subject";
    const NS_PREFIX: &'static str = "saml2";
    const NS_URI: &'static str = "urn:oasis:names:tc:SAML:2.0:assertion";

    #[inline]
    pub fn base_id(&self) -> Option<&BaseID> {
        self.base_id.as_ref()
    }

    #[inline]
    pub fn name_id(&self) -> Option<&NameID> {
        self.name_id.as_ref()
    }

    #[inline]
    pub fn encrypted_id(&self) -> Option<&EncryptedID> {
        self.encrypted_id.as_ref()
    }

    #[inline]
    pub fn subject_confirmations(&self) -> &[SubjectConfirmation] {
        &self.subject_confirmations
    }

    #[inline]
    pub fn set_base_id(&mut self, base_id: Option<BaseID>) {
        self.base_id = base_id;
    }

    #[inline]
    pub fn set_name_id(&mut self, name_id: Option<NameID>) {
        self.name_id = name_id;
    }

    #[inline]
    pub fn set_encrypted_id(&mut self, encrypted_id: Option<EncryptedID>) {
        self.encrypted_id = encrypted_id;
    }

    #[inline]
    pub fn add_subject_confirmation(&mut self, subject_confirmation: SubjectConfirmation) {
        self.subject_confirmations.push(subject_confirmation);
    }
}

impl TryFrom<Ref<'_, XmlObject>> for Subject {
    type Error = SAMLError;

    fn try_from(element: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        let mut subject = Subject::default();
        for child in element.children() {
            let child = child.borrow();
            match child.q_name().local_name() {
                Subject::CHILD_BASE_ID => {
                    subject.set_base_id(Some(BaseID::try_from(child)?));
                }
                Subject::CHILD_NAME_ID => {
                    subject.set_name_id(Some(NameID::try_from(child)?));
                }
                Subject::CHILD_ENCRYPTED_ID => {
                    subject.set_encrypted_id(Some(EncryptedID::try_from(child)?));
                }
                Subject::CHILD_SUBJECT_CONFIRMATION => {
                    subject.add_subject_confirmation(SubjectConfirmation::try_from(child)?);
                }
                _ => {}
            }
        }
        Ok(subject)
    }
}

impl TryFrom<Subject> for XmlObject {
    type Error = SAMLError;

    fn try_from(subject: Subject) -> Result<Self, Self::Error> {
        let mut xml_object = XmlObject::new(
            Some(Subject::NS_PREFIX.to_string()),
            Subject::ELEMENT_NAME.to_string(),
            Some(Subject::NS_URI.to_string()),
        );
        xml_object.add_namespace(Subject::NS_PREFIX.to_string(), Subject::NS_URI.to_string());
        if let Some(base_id) = subject.base_id {
            xml_object.add_child(Rc::new(RefCell::new(XmlObject::try_from(base_id)?)));
        }
        if let Some(name_id) = subject.name_id {
            xml_object.add_child(Rc::new(RefCell::new(XmlObject::try_from(name_id)?)));
        }
        if let Some(encrypted_id) = subject.encrypted_id {
            xml_object.add_child(Rc::new(RefCell::new(XmlObject::try_from(encrypted_id)?)));
        }
        for subject_confirmation in subject.subject_confirmations {
            xml_object.add_child(Rc::new(RefCell::new(XmlObject::try_from(
                subject_confirmation,
            )?)));
        }
        Ok(xml_object)
    }
}
