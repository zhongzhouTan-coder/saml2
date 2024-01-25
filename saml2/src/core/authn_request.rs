use super::{name_id_policy::NameIDPolicy, subject::Subject};

pub struct AuthnRequest {
    subject: Option<Subject>,
    name_id_policy: Option<NameIDPolicy>,
    conditions: Option<String>,
    requested_authn_context: Option<String>,
    scoping: Option<String>,
    force_authn: Option<bool>,
    is_passive: Option<bool>,
    assertion_consumer_service_index: Option<usize>,
    assertion_consumer_service_url: Option<String>,
    protocol_binding: Option<String>,
    attribute_consuming_service_index: Option<usize>,
    provider_name: Option<String>,
}
