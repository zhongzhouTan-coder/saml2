use super::{
    authn_context_class_ref::AuthnContextClassRef,
    authn_context_comparison_type_enumeration::AuthnContextComparisonTypeEnumeration,
    authn_context_decl_ref::AuthnContextDeclRef,
};

#[derive(Clone)]
pub struct RequestedAuthnContext {
    authn_context_class_refs: Vec<AuthnContextClassRef>,
    authn_context_decl_refs: Vec<AuthnContextDeclRef>,
    comparison: Option<AuthnContextComparisonTypeEnumeration>,
}
