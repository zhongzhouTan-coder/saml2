use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use chrono::{DateTime, Utc};

use crate::{common::SAML2Obj, error::SAMLError, xml::XmlObject};

use super::{
    authn_context::AuthnContext, parse_from_string, statement::Statement,
    subject_locality::SubjectLocality,
};

#[derive(Debug, Default)]
pub struct AuthnStatement {
    subject_locality: Option<SubjectLocality>,
    authn_context: AuthnContext,
    authn_instant: DateTime<Utc>,
    session_index: Option<String>,
    session_not_on_or_after: Option<DateTime<Utc>>,
}

impl SAML2Obj for AuthnStatement {}

impl Statement for AuthnStatement {}

impl AuthnStatement {
    const ATTRIB_SESSION_INDEX: &'static str = "SessionIndex";
    const ATTRIB_SESSION_NOT_ON_OR_AFTER: &'static str = "SessionNotOnOrAfter";
    const ATTRIB_AUTHN_INSTANT: &'static str = "AuthnInstant";

    const CHILD_SUBJECT_LOCALITY: &'static str = "SubjectLocality";
    const CHILD_AUTHN_CONTEXT: &'static str = "AuthnContext";

    const ELEMENT_NAME: &'static str = "AuthnStatement";
    const NS_PREFIX: &'static str = "saml2";
    const NS_URI: &'static str = "urn:oasis:names:tc:SAML:2.0:assertion";

    #[inline]
    pub fn subject_locality(&self) -> Option<&SubjectLocality> {
        self.subject_locality.as_ref()
    }

    #[inline]
    pub fn set_subject_locality(&mut self, subject_locality: Option<SubjectLocality>) {
        self.subject_locality = subject_locality;
    }

    #[inline]
    pub fn authn_context(&self) -> &AuthnContext {
        &self.authn_context
    }

    #[inline]
    pub fn set_authn_context(&mut self, authn_context: AuthnContext) {
        self.authn_context = authn_context;
    }

    #[inline]
    pub fn authn_instant(&self) -> &DateTime<Utc> {
        &self.authn_instant
    }

    #[inline]
    pub fn set_authn_instant(&mut self, authn_instant: Option<DateTime<Utc>>) {
        self.authn_instant = authn_instant.unwrap_or_else(Utc::now);
    }

    #[inline]
    pub fn session_index(&self) -> Option<&String> {
        self.session_index.as_ref()
    }

    #[inline]
    pub fn set_session_index(&mut self, session_index: Option<String>) {
        self.session_index = session_index;
    }

    #[inline]
    pub fn session_not_on_or_after(&self) -> Option<&DateTime<Utc>> {
        self.session_not_on_or_after.as_ref()
    }

    #[inline]
    pub fn set_session_not_on_or_after(&mut self, session_not_on_or_after: Option<DateTime<Utc>>) {
        self.session_not_on_or_after = session_not_on_or_after;
    }
}

impl TryFrom<Ref<'_, XmlObject>> for AuthnStatement {
    type Error = SAMLError;

    fn try_from(object: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        let mut authn_statement = AuthnStatement::default();
        for attribute in object.attributes() {
            let (key, value) = (attribute.0.as_str(), attribute.1.as_str());
            match key {
                AuthnStatement::ATTRIB_SESSION_INDEX => {
                    authn_statement.set_session_index(Some(value.to_string()));
                }
                AuthnStatement::ATTRIB_SESSION_NOT_ON_OR_AFTER => {
                    authn_statement.set_session_not_on_or_after(Some(parse_from_string(value)?));
                }
                AuthnStatement::ATTRIB_AUTHN_INSTANT => {
                    authn_statement.set_authn_instant(Some(parse_from_string(value)?))
                }
                _ => {}
            }
        }
        for child in object.children() {
            let child = child.borrow();
            match child.q_name().local_name() {
                AuthnStatement::CHILD_SUBJECT_LOCALITY => {
                    authn_statement.set_subject_locality(Some(SubjectLocality::try_from(child)?));
                }
                AuthnStatement::CHILD_AUTHN_CONTEXT => {
                    authn_statement.set_authn_context(AuthnContext::try_from(child)?);
                }
                _ => {}
            }
        }
        Ok(authn_statement)
    }
}
