use std::cell::Ref;

use chrono::{DateTime, Utc};

use crate::{error::SAMLError, xml::XmlObject};

use super::{authn_context::AuthnContext, parse_from_string, statement::Statement};

#[derive(Debug, Default)]
pub struct AuthnStatement {
    subject_locality: Option<String>,
    authn_context: Option<AuthnContext>,
    authn_instant: Option<DateTime<Utc>>,
    session_index: Option<String>,
    session_not_on_or_after: Option<DateTime<Utc>>,
}

impl Statement for AuthnStatement {}

/// implement getter and setter for AuthnStatement

impl AuthnStatement {
    const ATTRIB_SESSION_INDEX: &'static str = "SessionIndex";
    const ATTRIB_SESSION_NOT_ON_OR_AFTER: &'static str = "SessionNotOnOrAfter";

    const CHILD_SUBJECT_LOCALITY: &'static str = "SubjectLocality";
    const CHILD_AUTHN_CONTEXT: &'static str = "AuthnContext";
    const CHILD_AUTHN_INSTANT: &'static str = "AuthnInstant";
    const CHILD_SESSION_INDEX: &'static str = "SessionIndex";

    pub fn subject_locality(&self) -> Option<&String> {
        self.subject_locality.as_ref()
    }

    pub fn set_subject_locality(&mut self, subject_locality: Option<String>) {
        self.subject_locality = subject_locality
    }

    pub fn authn_context(&self) -> Option<&AuthnContext> {
        self.authn_context.as_ref()
    }

    pub fn set_authn_context(&mut self, authn_context: Option<AuthnContext>) {
        self.authn_context = authn_context
    }

    pub fn authn_instant(&self) -> Option<DateTime<Utc>> {
        self.authn_instant
    }

    pub fn set_authn_instant(&mut self, authn_instant: Option<DateTime<Utc>>) {
        self.authn_instant = authn_instant
    }

    pub fn session_index(&self) -> Option<&String> {
        self.session_index.as_ref()
    }

    pub fn set_session_index(&mut self, session_index: Option<String>) {
        self.session_index = session_index
    }

    pub fn session_not_on_or_after(&self) -> Option<DateTime<Utc>> {
        self.session_not_on_or_after
    }

    pub fn set_session_not_on_or_after(&mut self, session_not_on_or_after: Option<DateTime<Utc>>) {
        self.session_not_on_or_after = session_not_on_or_after
    }
}

/// implement tryFrom Ref<'_, XmlObject> for AuthnStatement
impl TryFrom<Ref<'_, XmlObject>> for AuthnStatement {
    type Error = SAMLError;

    fn try_from(object: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        let mut authn_statement = AuthnStatement::default();
        for attribute in object.attributes() {
            match attribute.0.as_str() {
                Self::ATTRIB_SESSION_INDEX => {
                    authn_statement.set_session_index(Some(attribute.1.to_string()));
                }
                Self::ATTRIB_SESSION_NOT_ON_OR_AFTER => {
                    authn_statement.set_session_not_on_or_after(Some(parse_from_string(
                        attribute.1.as_str(),
                    )?));
                }
                _ => {}
            }
        }
        Ok(authn_statement)
    }
}
