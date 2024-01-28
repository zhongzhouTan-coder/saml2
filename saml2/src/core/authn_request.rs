use std::time::Instant;

use chrono::{format::parse, DateTime, Utc};
use xml::{reader::XmlEvent, EventReader};

use crate::{
    error::SAMLError::{MessageDecodingError, UnmarshallingError},
    util::InputStream,
};

use super::{
    conditions::Conditions, extensions::Extensions, issuer::Issuer, name_id_policy::NameIDPolicy,
    request_abstract_type::RequestAbstractType, requested_authn_context::RequestedAuthnContext,
    saml_version::SAMLVersion, scoping::Scoping, subject::Subject,
};

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
    const FORCE_AUTHN_ATTRIBUTE_NAME: &'static str = "ForceAuthn";
    const IS_PASSIVE_ATTRIBUTE_NAME: &'static str = "IsPassive";
    const PROTOCOL_BINDING_ATTRIBUTE_NAME: &'static str = "ProtocolBinding";
    const ASSERTION_CONSUMER_SERVICE_INDEX_ATTRIBUTE_NAME: &'static str =
        "AssertionConsumerServiceIndex";
    const ASSERTION_CONSUMER_SERVICE_URL_ATTRIBUTE_NAME: &'static str =
        "AssertionConsumerServiceURL";
    const ATTRIBUTE_CONSUMING_SERVICE_INDEX_ATTRIBUTE_NAME: &'static str =
        "AttributeConsumingServiceIndex";
    const PROVIDER_NAME_ATTRIBUTE_NAME: &'static str = "ProviderName";
    const VERSION_ATTRIBUTE_NAME: &'static str = "Version";
    const ID_ATTRIBUTE_NAME: &'static str = "ID";
    const ISSUE_INSTANT_ATTRIBUTE_NAME: &'static str = "IssueInstant";
    const DESTINATION_ATTRIBUTE_NAME: &'static str = "Destination";
    const CONSENT_ATTRIBUTE_NAME: &'static str = "Consent";

    pub fn subject(&self) -> Option<&Subject> {
        self.subject.as_ref()
    }

    pub fn set_subject(&mut self, subject: Option<Subject>) {
        self.subject = subject
    }

    pub fn name_id_policy(&self) -> Option<&NameIDPolicy> {
        self.name_id_policy.as_ref()
    }

    pub fn set_name_id_policy(&mut self, name_id_policy: Option<NameIDPolicy>) {
        self.name_id_policy = name_id_policy
    }

    pub fn conditions(&self) -> Option<&Conditions> {
        self.conditions.as_ref()
    }

    pub fn set_conditions(&mut self, conditions: Option<Conditions>) {
        self.conditions = conditions
    }

    pub fn requested_authn_context(&self) -> Option<&RequestedAuthnContext> {
        self.requested_authn_context.as_ref()
    }

    pub fn set_request_authn_context(
        &mut self,
        request_authn_context: Option<RequestedAuthnContext>,
    ) {
        self.requested_authn_context = request_authn_context
    }

    pub fn scoping(&self) -> Option<&Scoping> {
        self.scoping.as_ref()
    }

    pub fn set_scoping(&mut self, scoping: Option<Scoping>) {
        self.scoping = scoping;
    }

    pub fn force_authn(&self) -> Option<bool> {
        self.force_authn
    }

    pub fn set_force_authn(&mut self, force_authn: Option<bool>) {
        self.force_authn = force_authn;
    }

    pub fn is_passive(&self) -> Option<bool> {
        self.is_passive
    }

    pub fn set_is_passive(&mut self, is_passive: Option<bool>) {
        self.is_passive = is_passive;
    }

    pub fn assertion_consumer_service_index(&self) -> Option<usize> {
        self.assertion_consumer_service_index
    }

    pub fn set_assertion_consumer_service_index(
        &mut self,
        assertion_consumer_service_index: Option<usize>,
    ) {
        self.assertion_consumer_service_index = assertion_consumer_service_index;
    }

    pub fn assertion_consumer_service_url(&self) -> Option<&String> {
        self.assertion_consumer_service_url.as_ref()
    }

    pub fn set_assertion_consumer_service_url(
        &mut self,
        assertion_consumer_service_url: Option<String>,
    ) {
        self.assertion_consumer_service_url = assertion_consumer_service_url;
    }

    pub fn protocol_binding(&self) -> Option<&String> {
        self.protocol_binding.as_ref()
    }

    pub fn set_protocol_binding(&mut self, protocol_binding: Option<String>) {
        self.protocol_binding = protocol_binding;
    }

    pub fn attribute_consuming_service_index(&self) -> Option<usize> {
        self.attribute_consuming_service_index
    }

    pub fn set_attribute_consuming_service_index(
        &mut self,
        attribute_consuming_service_index: Option<usize>,
    ) {
        self.attribute_consuming_service_index = attribute_consuming_service_index;
    }

    pub fn provider_name(&self) -> Option<&String> {
        self.provider_name.as_ref()
    }

    pub fn set_provider_name(&mut self, provider_name: Option<String>) {
        self.provider_name = provider_name;
    }
}

impl RequestAbstractType for AuthnRequest {
    fn version(&self) -> &SAMLVersion {
        &self.version
    }

    fn set_version(&mut self, saml_version: SAMLVersion) {
        self.version = saml_version
    }

    fn id(&self) -> &String {
        &self.id
    }

    fn set_id(&mut self, id: String) {
        self.id = id;
    }

    fn issue_instant(&self) -> &DateTime<Utc> {
        &self.issue_instant
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

    fn extensions(&self) -> Option<&Extensions> {
        self.extensions.as_ref()
    }

    fn set_extensions(&mut self, extensions: Option<Extensions>) {
        self.extensions = extensions
    }

    fn signature(&self) -> Option<&String> {
        self.signature.as_ref()
    }

    fn set_signature(&mut self, signature: Option<String>) {
        self.signature = signature
    }
}

impl Default for AuthnRequest {
    fn default() -> Self {
        Self {
            id: Default::default(),
            version: Default::default(),
            issue_instant: Utc::now(),
            destination: Default::default(),
            consent: Default::default(),
            issuer: Default::default(),
            extensions: Default::default(),
            signature: Default::default(),
            subject: Default::default(),
            name_id_policy: Default::default(),
            conditions: Default::default(),
            requested_authn_context: Default::default(),
            scoping: Default::default(),
            force_authn: Default::default(),
            is_passive: Default::default(),
            assertion_consumer_service_index: Default::default(),
            assertion_consumer_service_url: Default::default(),
            protocol_binding: Default::default(),
            attribute_consuming_service_index: Default::default(),
            provider_name: Default::default(),
        }
    }
}

impl TryFrom<EventReader<InputStream>> for AuthnRequest {
    type Error = crate::error::SAMLError;

    fn try_from(mut reader: EventReader<InputStream>) -> Result<Self, Self::Error> {
        let element = reader
            .next()
            .map_err(|err| MessageDecodingError(err.msg().to_string()))?;
        let mut authn_request = AuthnRequest::default();
        match element {
            XmlEvent::StartElement { attributes, .. } => {
                for attr in attributes {
                    let local_name = attr.name.local_name.as_str();
                    let value = attr.value.to_string();
                    match local_name {
                        Self::ID_ATTRIBUTE_NAME => authn_request.set_id(value),
                        Self::VERSION_ATTRIBUTE_NAME => {
                            authn_request.set_version(SAMLVersion::from_string(&value)?)
                        }
                        Self::ISSUE_INSTANT_ATTRIBUTE_NAME => {
                            let issue_instant = value.parse::<DateTime<Utc>>().map_err(|_| {
                                UnmarshallingError("unsupported issue instant format!".to_string())
                            })?;
                            authn_request.set_issue_instant(issue_instant)
                        }
                        Self::DESTINATION_ATTRIBUTE_NAME => {
                            authn_request.set_destination(Some(value))
                        }
                        Self::FORCE_AUTHN_ATTRIBUTE_NAME => {
                            let force_auth = value.parse::<bool>().map_err(|_| {
                                UnmarshallingError("unsupported force authn format!".to_string())
                            })?;
                            authn_request.set_force_authn(Some(force_auth))
                        }
                        Self::PROTOCOL_BINDING_ATTRIBUTE_NAME => {
                            authn_request.set_protocol_binding(Some(value))
                        }
                        Self::ASSERTION_CONSUMER_SERVICE_URL_ATTRIBUTE_NAME => {
                            authn_request.set_assertion_consumer_service_url(Some(value))
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        Ok(AuthnRequest::default())
    }
}
