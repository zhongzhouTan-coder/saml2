use std::str::FromStr;

use crate::error::SAMLError;

mod abstract_name_id_type;
mod advice;
mod assertion;
mod audience;
mod audience_restriction;
mod authenticating_authority;
mod authn_context;
mod authn_context_class_ref;
mod authn_context_comparison_type_enumeration;
mod authn_context_decl;
mod authn_context_decl_ref;
pub mod authn_request;
mod authn_statement;
mod base_id;
mod conditions;
mod element_type;
mod encrypted_assertion;
mod encrypted_data;
mod encrypted_element_type;
mod encrypted_id;
mod encrypted_key;
mod extensions;
mod idp_entry;
mod idp_list;
mod issuer;
mod name_id;
mod name_id_policy;
pub mod request_abstract_type;
mod requested_authn_context;
mod requester_id;
pub mod response;
mod saml_version;
mod scoping;
mod statement;
mod status;
mod status_code;
pub mod status_response_type;
mod subject;
mod subject_confirmation;
mod subject_confirmation_data;

/// parse a xml string to a type that implements the fromStr trait
fn parse_from_string<T: FromStr>(value: &str) -> Result<T, SAMLError> {
    T::from_str(value).map_err(|_| SAMLError::UnmarshallingError("Invalid XML".to_string()))
}
