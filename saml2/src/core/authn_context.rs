use super::{
    authenticating_authority::AuthenticatingAuthority,
    authn_context_class_ref::AuthnContextClassRef, authn_context_decl::AuthnContextDecl,
    authn_context_decl_ref::AuthnContextDeclRef,
};

#[derive(Debug)]
pub struct AuthnContext {
    authn_context_class_ref: Option<AuthnContextClassRef>,
    auth_context_decl: Option<AuthnContextDecl>,
    authn_context_decl_ref: Option<AuthnContextDeclRef>,
    authenticating_authorities: Vec<AuthenticatingAuthority>,
}
