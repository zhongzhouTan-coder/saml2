use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use crate::{common::SAML2Obj, core::status_code::StatusCode, error::SAMLError, xml::XmlObject};

use super::{status_detail::StatusDetail, status_message::StatusMessage};

#[derive(Default, Debug)]
pub struct Status {
    status_code: StatusCode,
    status_message: Option<StatusMessage>,
    status_detail: Option<StatusDetail>,
}

impl SAML2Obj for Status {}

impl Status {
    const CHILD_STATUS_CODE: &'static str = "StatusCode";
    const CHILD_STATUS_MESSAGE: &'static str = "StatusMessage";
    const CHILD_STATUS_DETAIL: &'static str = "StatusDetail";

    const ELEMENT_NAME: &'static str = "Status";
    const NS_PREFIX: &'static str = "saml2p";
    const NS_URI: &'static str = "urn:oasis:names:tc:SAML:2.0:protocol";

    pub fn status_code(&self) -> &StatusCode {
        &self.status_code
    }

    pub fn set_status_code(&mut self, status_code: StatusCode) {
        self.status_code = status_code;
    }

    pub fn status_message(&self) -> Option<&StatusMessage> {
        self.status_message.as_ref()
    }

    pub fn set_status_message(&mut self, status_message: Option<StatusMessage>) {
        self.status_message = status_message;
    }

    pub fn status_detail(&self) -> Option<&StatusDetail> {
        self.status_detail.as_ref()
    }

    pub fn set_status_detail(&mut self, status_detail: Option<StatusDetail>) {
        self.status_detail = status_detail;
    }
}

impl TryFrom<Ref<'_, XmlObject>> for Status {
    type Error = SAMLError;

    fn try_from(object: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        let mut status = Status::default();
        for child in object.children() {
            let child = child.borrow();
            match child.q_name().local_name() {
                Status::CHILD_STATUS_CODE => {
                    status.set_status_code(StatusCode::try_from(child)?);
                }
                Status::CHILD_STATUS_DETAIL => {
                    todo!("StatusDetail not implemented yet")
                }
                Status::CHILD_STATUS_MESSAGE => {
                    todo!("StatusMessage not implemented yet")
                }
                _ => {}
            }
        }
        Ok(status)
    }
}

impl TryFrom<Status> for XmlObject {
    type Error = SAMLError;

    fn try_from(value: Status) -> Result<Self, Self::Error> {
        let mut object = XmlObject::new(
            Some(Status::NS_URI.to_string()),
            Status::ELEMENT_NAME.to_string(),
            Some(Status::NS_PREFIX.to_string()),
        );
        object.add_namespace(Status::NS_PREFIX.to_string(), Status::NS_URI.to_string());
        object.add_child(Rc::new(RefCell::new(XmlObject::try_from(
            value.status_code,
        )?)));
        if let Some(status_message) = value.status_message {
            todo!("StatusMessage not implemented yet")
        }
        if let Some(status_detail) = value.status_detail {
            todo!("StatusDetail not implemented yet")
        }
        Ok(object)
    }
}
