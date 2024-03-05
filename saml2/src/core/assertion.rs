use std::cell::Ref;

use chrono::{DateTime, Utc};

use crate::{common::SAML2Obj, error::SAMLError, xml::XmlObject};

use super::{
    advice::Advice, authn_statement::AuthnStatement, conditions::Conditions, issuer::Issuer,
    parse_from_string, saml_version::SAMLVersion, statement::Statement, subject::Subject,
};

#[derive(Debug, Default)]
pub struct Assertion {
    id: String,
    version: SAMLVersion,
    issue_instant: DateTime<Utc>,
    issuer: Issuer,
    signature: Option<String>,
    subject: Option<Subject>,
    conditions: Option<Conditions>,
    advice: Option<Advice>,
    statements: Vec<Box<dyn Statement>>,
}

impl SAML2Obj for Assertion {}

impl Assertion {
    const ATTRIB_ID: &'static str = "ID";
    const ATTRIB_VERSION: &'static str = "Version";
    const ATTRIB_ISSUE_INSTANT: &'static str = "IssueInstant";

    const CHILD_ISSUER: &'static str = "Issuer";
    const CHILD_SIGNATURE: &'static str = "Signature";
    const CHILD_SUBJECT: &'static str = "Subject";
    const CHILD_CONDITIONS: &'static str = "Conditions";
    const CHILD_ADVICE: &'static str = "Advice";
    const CHILD_AUTHN_STATEMENT: &'static str = "AuthnStatement";
    const CHILD_ATTRIBUTE_STATEMENT: &'static str = "AttributeStatement";

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn version(&self) -> &SAMLVersion {
        &self.version
    }

    pub fn set_version(&mut self, version: SAMLVersion) {
        self.version = version
    }

    pub fn issue_instant(&self) -> DateTime<Utc> {
        self.issue_instant
    }

    pub fn set_issue_instant(&mut self, issue_instant: DateTime<Utc>) {
        self.issue_instant = issue_instant
    }

    pub fn issuer(&self) -> &Issuer {
        &self.issuer
    }

    pub fn set_issuer(&mut self, issuer: Issuer) {
        self.issuer = issuer
    }

    pub fn signature(&self) -> Option<&String> {
        self.signature.as_ref()
    }

    pub fn set_signature(&mut self, signature: Option<String>) {
        self.signature = signature
    }

    pub fn subject(&self) -> Option<&Subject> {
        self.subject.as_ref()
    }

    pub fn set_subject(&mut self, subject: Option<Subject>) {
        self.subject = subject
    }

    pub fn conditions(&self) -> Option<&Conditions> {
        self.conditions.as_ref()
    }

    pub fn set_conditions(&mut self, conditions: Option<Conditions>) {
        self.conditions = conditions
    }

    pub fn advice(&self) -> Option<&Advice> {
        self.advice.as_ref()
    }

    pub fn set_advice(&mut self, advice: Option<Advice>) {
        self.advice = advice
    }

    pub fn statements(&self) -> &Vec<Box<dyn Statement>> {
        &self.statements
    }

    pub fn set_statements(&mut self, statements: Vec<Box<dyn Statement>>) {
        self.statements = statements
    }

    pub fn add_statement(&mut self, statement: Box<dyn Statement>) {
        self.statements.push(statement)
    }
}

impl TryFrom<Ref<'_, XmlObject>> for Assertion {
    type Error = SAMLError;

    fn try_from(object: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        let mut assertion = Assertion::default();
        for attribute in object.attributes() {
            match attribute.0.as_str() {
                Self::ATTRIB_ID => {
                    assertion.set_id(attribute.1.to_string());
                }
                Self::ATTRIB_VERSION => {
                    assertion.set_version(SAMLVersion::from_string(attribute.1.as_str())?);
                }
                Self::ATTRIB_ISSUE_INSTANT => {
                    assertion.set_issue_instant(parse_from_string::<DateTime<Utc>>(
                        attribute.1.as_str(),
                    )?);
                }

                _ => {}
            }
        }
        for child in object.children() {
            let child = child.borrow();
            match child.q_name().local_name() {
                Self::CHILD_ISSUER => {
                    assertion.set_issuer(Issuer::try_from(child)?);
                }
                Self::CHILD_SIGNATURE => {
                    assertion.set_signature(Some(child.to_string()));
                }
                Self::CHILD_SUBJECT => {
                    assertion.set_subject(Some(Subject::try_from(child)?));
                }
                Self::CHILD_CONDITIONS => {
                    assertion.set_conditions(Some(Conditions::try_from(child)?));
                }
                Self::CHILD_ADVICE => {
                    // todo
                }
                Self::CHILD_AUTHN_STATEMENT => {
                    assertion.add_statement(
                        Box::new(AuthnStatement::try_from(child)?) as Box<dyn Statement>
                    );
                }
                Self::CHILD_ATTRIBUTE_STATEMENT => {
                    // todo
                }
                _ => {}
            }
        }
        Ok(assertion)
    }
}
