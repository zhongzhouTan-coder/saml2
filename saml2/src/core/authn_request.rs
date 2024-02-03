use std::cell::Ref;

use chrono::{DateTime, Utc};

use crate::core::parse_from_string;
use crate::{error::SAMLError, xml::XmlObject};

use super::{
    conditions::Conditions, extensions::Extensions, issuer::Issuer, name_id_policy::NameIDPolicy,
    request_abstract_type::RequestAbstractType, requested_authn_context::RequestedAuthnContext,
    saml_version::SAMLVersion, scoping::Scoping, subject::Subject,
};

#[derive(Debug, Default)]
pub struct AuthnRequest {
    id: String,
    version: SAMLVersion,
    issue_instant: DateTime<Utc>,
    destination: Option<String>,
    consent: Option<String>,
    issuer: Option<Issuer>,
    extensions: Option<Extensions>,
    signature: Option<String>,
    subject: Option<Subject>,
    name_id_policy: Option<NameIDPolicy>,
    conditions: Option<Conditions>,
    requested_authn_context: Option<RequestedAuthnContext>,
    scoping: Option<Scoping>,
    force_authn: Option<bool>,
    is_passive: Option<bool>,
    assertion_consumer_service_index: Option<usize>,
    assertion_consumer_service_url: Option<String>,
    protocol_binding: Option<String>,
    attribute_consuming_service_index: Option<usize>,
    provider_name: Option<String>,
}

impl AuthnRequest {
    const ATTRIB_FORCE_AUTHN: &'static str = "ForceAuthn";
    const ATTRIB_IS_PASSIVE: &'static str = "IsPassive";
    const ATTRIB_PROTOCOL_BINDING: &'static str = "ProtocolBinding";
    const ATTRIB_ASSERTION_CONSUMER_SERVICE_INDEX: &'static str = "AssertionConsumerServiceIndex";
    const ATTRIB_ASSERTION_CONSUMER_SERVICE_URL: &'static str = "AssertionConsumerServiceURL";
    const ATTRIB_ATTRIBUTE_CONSUMING_SERVICE_INDEX: &'static str = "AttributeConsumingServiceIndex";
    const ATTRIB_PROVIDER_NAME: &'static str = "ProviderName";
    const ATTRIB_VERSION: &'static str = "Version";
    const ATTRIB_ID: &'static str = "ID";
    const ATTRIB_ISSUE_INSTANT: &'static str = "IssueInstant";
    const ATTRIB_DESTINATION: &'static str = "Destination";
    const ATTRIB_CONSENT: &'static str = "Consent";

    const CHILD_ISSUER: &'static str = "Issuer";
    const CHILD_EXTENSIONS: &'static str = "Extensions";
    const CHILD_NAME_ID_POLICY: &'static str = "NameIDPolicy";
    const CHILD_SUBJECT: &'static str = "Subject";
    const CHILD_CONDITIONS: &'static str = "Conditions";
    const CHILD_REQUESTED_AUTHN_CONTEXT: &'static str = "RequestedAuthnContext";
    const CHILD_SCOPING: &'static str = "Scoping";
    const CHILD_SIGNATURE: &'static str = "Signature";

    #[inline]
    pub fn subject(&self) -> Option<&Subject> {
        self.subject.as_ref()
    }

    #[inline]
    pub fn set_subject(&mut self, subject: Option<Subject>) {
        self.subject = subject
    }

    #[inline]
    pub fn name_id_policy(&self) -> Option<&NameIDPolicy> {
        self.name_id_policy.as_ref()
    }

    #[inline]
    pub fn set_name_id_policy(&mut self, name_id_policy: Option<NameIDPolicy>) {
        self.name_id_policy = name_id_policy
    }

    #[inline]
    pub fn conditions(&self) -> Option<&Conditions> {
        self.conditions.as_ref()
    }

    #[inline]
    pub fn set_conditions(&mut self, conditions: Option<Conditions>) {
        self.conditions = conditions
    }

    #[inline]
    pub fn requested_authn_context(&self) -> Option<&RequestedAuthnContext> {
        self.requested_authn_context.as_ref()
    }

    #[inline]
    pub fn set_requested_authn_context(
        &mut self,
        request_authn_context: Option<RequestedAuthnContext>,
    ) {
        self.requested_authn_context = request_authn_context
    }

    #[inline]
    pub fn scoping(&self) -> Option<&Scoping> {
        self.scoping.as_ref()
    }

    #[inline]
    pub fn set_scoping(&mut self, scoping: Option<Scoping>) {
        self.scoping = scoping;
    }

    #[inline]
    pub fn force_authn(&self) -> Option<bool> {
        self.force_authn
    }

    #[inline]
    pub fn set_force_authn(&mut self, force_authn: Option<bool>) {
        self.force_authn = force_authn;
    }

    #[inline]
    pub fn is_passive(&self) -> Option<bool> {
        self.is_passive
    }

    #[inline]
    pub fn set_is_passive(&mut self, is_passive: Option<bool>) {
        self.is_passive = is_passive;
    }

    #[inline]
    pub fn assertion_consumer_service_index(&self) -> Option<usize> {
        self.assertion_consumer_service_index
    }

    #[inline]
    pub fn set_assertion_consumer_service_index(
        &mut self,
        assertion_consumer_service_index: Option<usize>,
    ) {
        self.assertion_consumer_service_index = assertion_consumer_service_index;
    }

    #[inline]
    pub fn assertion_consumer_service_url(&self) -> Option<&String> {
        self.assertion_consumer_service_url.as_ref()
    }

    #[inline]
    pub fn set_assertion_consumer_service_url(
        &mut self,
        assertion_consumer_service_url: Option<String>,
    ) {
        self.assertion_consumer_service_url = assertion_consumer_service_url;
    }

    #[inline]
    pub fn protocol_binding(&self) -> Option<&String> {
        self.protocol_binding.as_ref()
    }

    #[inline]
    pub fn set_protocol_binding(&mut self, protocol_binding: Option<String>) {
        self.protocol_binding = protocol_binding;
    }

    #[inline]
    pub fn attribute_consuming_service_index(&self) -> Option<usize> {
        self.attribute_consuming_service_index
    }

    #[inline]
    pub fn set_attribute_consuming_service_index(
        &mut self,
        attribute_consuming_service_index: Option<usize>,
    ) {
        self.attribute_consuming_service_index = attribute_consuming_service_index;
    }

    #[inline]
    pub fn provider_name(&self) -> Option<&String> {
        self.provider_name.as_ref()
    }

    #[inline]
    pub fn set_provider_name(&mut self, provider_name: Option<String>) {
        self.provider_name = provider_name;
    }
}

impl RequestAbstractType for AuthnRequest {
    #[inline]
    fn version(&self) -> &SAMLVersion {
        &self.version
    }

    #[inline]
    fn set_version(&mut self, saml_version: SAMLVersion) {
        self.version = saml_version
    }

    #[inline]
    fn id(&self) -> &String {
        &self.id
    }

    #[inline]
    fn set_id(&mut self, id: String) {
        self.id = id;
    }

    #[inline]
    fn issue_instant(&self) -> &DateTime<Utc> {
        &self.issue_instant
    }

    #[inline]
    fn set_issue_instant(&mut self, issue_instant: DateTime<Utc>) {
        self.issue_instant = issue_instant
    }

    #[inline]
    fn destination(&self) -> Option<&String> {
        self.destination.as_ref()
    }

    #[inline]
    fn set_destination(&mut self, destination: Option<String>) {
        self.destination = destination
    }

    #[inline]
    fn consent(&self) -> Option<&String> {
        self.consent.as_ref()
    }

    #[inline]
    fn set_consent(&mut self, consent: Option<String>) {
        self.consent = consent
    }

    #[inline]
    fn issuer(&self) -> Option<&Issuer> {
        self.issuer.as_ref()
    }

    #[inline]
    fn set_issuer(&mut self, issuer: Option<Issuer>) {
        self.issuer = issuer
    }

    #[inline]
    fn extensions(&self) -> Option<&Extensions> {
        self.extensions.as_ref()
    }

    #[inline]
    fn set_extensions(&mut self, extensions: Option<Extensions>) {
        self.extensions = extensions
    }

    #[inline]
    fn signature(&self) -> Option<&String> {
        self.signature.as_ref()
    }

    #[inline]
    fn set_signature(&mut self, signature: Option<String>) {
        self.signature = signature
    }
}

impl TryFrom<Ref<'_, XmlObject>> for AuthnRequest {
    type Error = SAMLError;

    fn try_from(element: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        let mut authn_request = AuthnRequest::default();
        for attribute in element.attributes() {
            let (key, value) = (attribute.0.as_str(), attribute.1.as_str());
            match key {
                AuthnRequest::ATTRIB_VERSION => {
                    authn_request.set_version(SAMLVersion::from_string(value)?);
                }
                AuthnRequest::ATTRIB_ID => {
                    authn_request.set_id(value.to_string());
                }
                AuthnRequest::ATTRIB_ISSUE_INSTANT => {
                    authn_request.set_issue_instant(parse_from_string(value)?);
                }
                AuthnRequest::ATTRIB_DESTINATION => {
                    authn_request.set_destination(Some(value.to_string()));
                }
                AuthnRequest::ATTRIB_CONSENT => {
                    authn_request.set_consent(Some(value.to_string()));
                }
                AuthnRequest::ATTRIB_ATTRIBUTE_CONSUMING_SERVICE_INDEX => {
                    authn_request
                        .set_attribute_consuming_service_index(Some(parse_from_string(value)?));
                }
                AuthnRequest::ATTRIB_PROVIDER_NAME => {
                    authn_request.set_provider_name(Some(value.to_string()));
                }
                AuthnRequest::ATTRIB_ASSERTION_CONSUMER_SERVICE_INDEX => {
                    authn_request
                        .set_assertion_consumer_service_index(Some(parse_from_string(value)?));
                }
                AuthnRequest::ATTRIB_ASSERTION_CONSUMER_SERVICE_URL => {
                    authn_request.set_assertion_consumer_service_url(Some(value.to_string()));
                }
                AuthnRequest::ATTRIB_PROTOCOL_BINDING => {
                    authn_request.set_protocol_binding(Some(value.to_string()));
                }
                AuthnRequest::ATTRIB_FORCE_AUTHN => {
                    authn_request.set_force_authn(Some(parse_from_string(value)?));
                }
                AuthnRequest::ATTRIB_IS_PASSIVE => {
                    authn_request.set_is_passive(Some(parse_from_string(value)?));
                }
                _ => {}
            }
        }
        for child in element.children() {
            let child = child.borrow();
            match child.q_name().local_name() {
                AuthnRequest::CHILD_ISSUER => {
                    authn_request.set_issuer(Some(Issuer::try_from(child)?));
                }
                AuthnRequest::CHILD_SUBJECT => {
                    authn_request.set_subject(Some(Subject::try_from(child)?));
                }
                AuthnRequest::CHILD_NAME_ID_POLICY => {
                    authn_request.set_name_id_policy(Some(NameIDPolicy::try_from(child)?));
                }
                AuthnRequest::CHILD_CONDITIONS => {
                    authn_request.set_conditions(Some(Conditions::try_from(child)?));
                }
                AuthnRequest::CHILD_REQUESTED_AUTHN_CONTEXT => {
                    authn_request
                        .set_requested_authn_context(Some(RequestedAuthnContext::try_from(child)?));
                }
                AuthnRequest::CHILD_SCOPING => {
                    authn_request.set_scoping(Some(Scoping::try_from(child)?));
                }
                AuthnRequest::CHILD_EXTENSIONS => {
                    authn_request.set_extensions(Some(Extensions::try_from(child)?));
                }
                AuthnRequest::CHILD_SIGNATURE => {}
                _ => {}
            }
        }
        Ok(authn_request)
    }
}
