use std::cell::Ref;

use crate::{error::SAMLError, xml::XmlObject};

use super::{
    authn_context_class_ref::AuthnContextClassRef,
    authn_context_comparison_type_enumeration::AuthnContextComparisonTypeEnumeration,
    authn_context_decl_ref::AuthnContextDeclRef,
};

#[derive(Clone, Default, Debug)]
pub struct RequestedAuthnContext {
    authn_context_class_refs: Vec<AuthnContextClassRef>,
    authn_context_decl_refs: Vec<AuthnContextDeclRef>,
    comparison: Option<AuthnContextComparisonTypeEnumeration>,
}

impl TryFrom<Ref<'_, XmlObject>> for RequestedAuthnContext {
    type Error = SAMLError;

    fn try_from(element: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        let mut requested_authn_context = RequestedAuthnContext::default();
        for child in element.children() {
            let child = child.borrow();
            match child.q_name().local_name() {
                _ => {
                    println!("child is {}", child.q_name())
                }
            }
        }
        Ok(requested_authn_context)
    }
}
