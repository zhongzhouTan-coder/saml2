use std::cell::Ref;

use chrono::{DateTime, Utc};

use crate::core::extensions::Extensions;
use crate::core::issuer::Issuer;
use crate::core::saml_version::SAMLVersion;
use crate::core::status::Status;
use crate::core::status_response_type::StatusResponseType;
use crate::error::SAMLError;
use crate::xml::XmlObject;

use super::assertion::Assertion;
use super::encrypted_assertion::EncryptedAssertion;
use super::parse_from_string;

#[derive(Debug)]
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
    assertions: Vec<Assertion>,
    encrypted_assertions: Vec<EncryptedAssertion>,
}

impl Response {
    const ATTRIB_ID: &'static str = "ID";
    const ATTRIB_IN_RESPONSE_TO: &'static str = "InResponseTo";
    const ATTRIB_VERSION: &'static str = "Version";
    const ATTRIB_ISSUE_INSTANT: &'static str = "IssueInstant";
    const ATTRIB_DESTINATION: &'static str = "Destination";
    const ATTRIB_CONSENT: &'static str = "Consent";

    pub fn assertions(&self) -> &Vec<Assertion> {
        self.assertions.as_ref()
    }

    pub fn add_assertion(&mut self, assertion: Assertion) {
        self.assertions.push(assertion)
    }

    pub fn encrypted_assertions(&self) -> &Vec<EncryptedAssertion> {
        self.encrypted_assertions.as_ref()
    }

    pub fn add_encrypted_assertion(&mut self, encrypted_assertion: EncryptedAssertion) {
        self.encrypted_assertions.push(encrypted_assertion)
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
        Response {
            id: String::new(),
            in_response_to: None,
            version: SAMLVersion::default(),
            issue_instant: Utc::now(),
            destination: None,
            consent: None,
            issuer: None,
            signature: None,
            extensions: None,
            status: Status::default(),
            assertions: Vec::new(),
            encrypted_assertions: Vec::new(),
        }
    }
}

impl TryFrom<Ref<'_, XmlObject>> for Response {
    type Error = SAMLError;

    fn try_from(object: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        let mut response = Response::default();
        for attribute in object.attributes() {
            match attribute.0.as_str() {
                Self::ATTRIB_ID => {
                    response.set_id(attribute.1.to_string());
                }
                Self::ATTRIB_IN_RESPONSE_TO => {
                    response.set_in_response_to(Some(attribute.1.to_string()));
                }
                Self::ATTRIB_VERSION => {
                    response.set_version(SAMLVersion::from_string(attribute.1.as_str())?);
                }
                Self::ATTRIB_ISSUE_INSTANT => {
                    response.set_issue_instant(parse_from_string::<DateTime<Utc>>(
                        attribute.1.as_str(),
                    )?);
                }
                Self::ATTRIB_DESTINATION => {
                    response.set_destination(Some(attribute.1.to_string()));
                }
                Self::ATTRIB_CONSENT => {
                    response.set_consent(Some(attribute.1.to_string()));
                }
                _ => {}
            }
        }
        for child in object.children() {
            let child = child.borrow();
            match child.q_name().local_name() {
                "Issuer" => {
                    response.set_issuer(Some(Issuer::try_from(child)?));
                }
                "Extensions" => {
                    response.set_extensions(Some(Extensions::try_from(child)?));
                }
                "Status" => {
                    response.set_status(Status::try_from(child)?);
                }
                "Assertion" => response.add_assertion(Assertion::try_from(child)?),
                "EncryptedAssertion" => {
                    // todo
                }
                _ => {}
            }
        }
        Ok(response)
    }
}
