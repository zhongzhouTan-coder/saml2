use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use crate::{common::SAML2Obj, error::SAMLError, xml::XmlObject};

use super::{
    base_id::BaseID, encrypted_id::EncryptedID, name_id::NameID,
    subject_confirmation_data::SubjectConfirmationData,
};

#[derive(Debug, Default)]
pub struct SubjectConfirmation {
    base_id: Option<BaseID>,
    name_id: Option<NameID>,
    encrypted_id: Option<EncryptedID>,
    subject_confirmation_data: Option<SubjectConfirmationData>,
    method: String,
}

impl SAML2Obj for SubjectConfirmation {}

impl SubjectConfirmation {
    const ATTRIB_METHOD: &'static str = "Method";

    const CHILD_BASE_ID: &'static str = "BaseID";
    const CHILD_NAME_ID: &'static str = "NameID";
    const CHILD_ENCRYPTED_ID: &'static str = "EncryptedID";
    const CHILD_SUBJECT_CONFIRMATION_DATA: &'static str = "SubjectConfirmationData";

    const ELEMENT_NAME: &'static str = "SubjectConfirmation";
    const NS_PREFIX: &'static str = "saml2";
    const NS_URI: &'static str = "urn:oasis:names:tc:SAML:2.0:assertion";

    #[inline]
    pub fn base_id(&self) -> Option<&BaseID> {
        self.base_id.as_ref()
    }

    #[inline]
    pub fn set_base_id(&mut self, base_id: Option<BaseID>) {
        self.base_id = base_id;
    }

    #[inline]
    pub fn name_id(&self) -> Option<&NameID> {
        self.name_id.as_ref()
    }

    #[inline]
    pub fn set_name_id(&mut self, name_id: Option<NameID>) {
        self.name_id = name_id;
    }

    #[inline]
    pub fn encrypted_id(&self) -> Option<&EncryptedID> {
        self.encrypted_id.as_ref()
    }

    #[inline]
    pub fn set_encrypted_id(&mut self, encrypted_id: Option<EncryptedID>) {
        self.encrypted_id = encrypted_id;
    }

    #[inline]
    pub fn subject_confirmation_data(&self) -> Option<&SubjectConfirmationData> {
        self.subject_confirmation_data.as_ref()
    }

    #[inline]
    pub fn set_subject_confirmation_data(
        &mut self,
        subject_confirmation_data: Option<SubjectConfirmationData>,
    ) {
        self.subject_confirmation_data = subject_confirmation_data;
    }

    #[inline]
    pub fn method(&self) -> &str {
        &self.method
    }

    #[inline]
    pub fn set_method(&mut self, method: String) {
        self.method = method;
    }
}

impl TryFrom<Ref<'_, XmlObject>> for SubjectConfirmation {
    type Error = SAMLError;

    fn try_from(element: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        let mut subject_confirmation = SubjectConfirmation::default();
        for attrib in element.attributes() {
            match attrib.0.as_str() {
                SubjectConfirmation::ATTRIB_METHOD => {
                    subject_confirmation.set_method(attrib.1.to_string());
                }
                _ => {}
            }
        }
        for child in element.children() {
            let child = child.borrow();
            match child.q_name().local_name() {
                SubjectConfirmation::CHILD_BASE_ID => {
                    subject_confirmation.set_base_id(Some(BaseID::try_from(child)?));
                }
                SubjectConfirmation::CHILD_NAME_ID => {
                    subject_confirmation.set_name_id(Some(NameID::try_from(child)?));
                }
                SubjectConfirmation::CHILD_ENCRYPTED_ID => {
                    subject_confirmation.set_encrypted_id(Some(EncryptedID::try_from(child)?));
                }
                SubjectConfirmation::CHILD_SUBJECT_CONFIRMATION_DATA => {
                    subject_confirmation.set_subject_confirmation_data(Some(
                        SubjectConfirmationData::try_from(child)?,
                    ));
                }
                _ => {}
            }
        }
        Ok(subject_confirmation)
    }
}

impl TryFrom<SubjectConfirmation> for XmlObject {
    type Error = SAMLError;

    fn try_from(subject_confirmation: SubjectConfirmation) -> Result<Self, Self::Error> {
        let mut xml_object = XmlObject::new(
            Some(SubjectConfirmation::NS_PREFIX.to_string()),
            SubjectConfirmation::ELEMENT_NAME.to_string(),
            Some(SubjectConfirmation::NS_URI.to_string()),
        );
        xml_object.add_namespace(
            SubjectConfirmation::NS_PREFIX.to_string(),
            SubjectConfirmation::NS_URI.to_string(),
        );
        xml_object.add_attribute(
            SubjectConfirmation::ATTRIB_METHOD.to_string(),
            subject_confirmation.method,
        );
        if let Some(base_id) = subject_confirmation.base_id {
            xml_object.add_child(Rc::new(RefCell::new(XmlObject::try_from(base_id)?)));
        }
        if let Some(name_id) = subject_confirmation.name_id {
            xml_object.add_child(Rc::new(RefCell::new(XmlObject::try_from(name_id)?)));
        }
        if let Some(encrypted_id) = subject_confirmation.encrypted_id {
            xml_object.add_child(Rc::new(RefCell::new(XmlObject::try_from(encrypted_id)?)));
        }
        if let Some(subject_confirmation_data) = subject_confirmation.subject_confirmation_data {
            xml_object.add_child(Rc::new(RefCell::new(XmlObject::try_from(
                subject_confirmation_data,
            )?)));
        }
        Ok(xml_object)
    }
}
