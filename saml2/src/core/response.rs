use std::cell::Ref;

use chrono::{DateTime, Utc};

use crate::core::extensions::Extensions;
use crate::core::issuer::Issuer;
use crate::core::saml_version::SAMLVersion;
use crate::core::status::Status;
use crate::core::status_response_type::StatusResponseType;
use crate::error::SAMLError;
use crate::xml::XmlObject;

pub struct Response {
    id: String,
    in_response_to: Option<String>,
    version: SAMLVersion,
    issue_instant: DateTime<Utc>,
    destination: Option<String>,
    consent: Option<String>,
    issuer: Option<Issuer>,
    signature: Option<String>,
    extensions: Option<Extensions>,
    status: Status,
    assertions: Vec<String>,
    encrypted_assertions: Vec<String>,
}

impl Response {
    pub fn assertions(&self) -> &Vec<String> {
        self.assertions.as_ref()
    }

    pub fn set_assertions(&mut self, assertions: Vec<String>) {
        self.assertions = assertions
    }

    pub fn encrypted_assertions(&self) -> &Vec<String> {
        self.encrypted_assertions.as_ref()
    }

    pub fn set_encrypted_assertions(&mut self, encrypted_assertions: Vec<String>) {
        self.encrypted_assertions = encrypted_assertions
    }
}

impl StatusResponseType for Response {
    fn id(&self) -> &String {
        &self.id
    }

    fn set_id(&mut self, id: String) {
        self.id = id;
    }

    fn in_response_to(&self) -> Option<&String> {
        self.in_response_to.as_ref()
    }

    fn set_in_response_to(&mut self, in_response_to: Option<String>) {
        self.in_response_to = in_response_to
    }

    fn version(&self) -> &SAMLVersion {
        &self.version
    }

    fn set_version(&mut self, version: SAMLVersion) {
        self.version = version
    }

    fn issue_instant(&self) -> DateTime<Utc> {
        self.issue_instant
    }

    fn set_issue_instant(&mut self, issue_instant: DateTime<Utc>) {
        self.issue_instant = issue_instant
    }

    fn destination(&self) -> Option<&String> {
        self.destination.as_ref()
    }

    fn set_destination(&mut self, destination: Option<String>) {
        self.destination = destination
    }

    fn consent(&self) -> Option<&String> {
        self.consent.as_ref()
    }

    fn set_consent(&mut self, consent: Option<String>) {
        self.consent = consent
    }

    fn issuer(&self) -> Option<&Issuer> {
        self.issuer.as_ref()
    }

    fn set_issuer(&mut self, issuer: Option<Issuer>) {
        self.issuer = issuer
    }

    fn signature(&self) -> Option<&String> {
        self.signature.as_ref()
    }

    fn set_signature(&mut self, signature: Option<String>) {
        self.signature = signature
    }

    fn extensions(&self) -> Option<&Extensions> {
        self.extensions.as_ref()
    }

    fn set_extensions(&mut self, extensions: Option<Extensions>) {
        self.extensions = extensions
    }

    fn status(&self) -> &Status {
        &self.status
    }

    fn set_status(&mut self, status: Status) {
        self.status = status
    }
}

impl Default for Response {
    fn default() -> Self {
        todo!()
    }
}

impl TryFrom<Ref<'_, XmlObject>> for Response {
    type Error = SAMLError;

    fn try_from(object: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        let mut response = Response::default();
        for attribute in object.attributes() {

        }
        todo!()
    }
}