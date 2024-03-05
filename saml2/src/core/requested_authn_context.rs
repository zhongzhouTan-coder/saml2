use std::{
    cell::{Ref, RefCell},
    rc::Rc,
    str::FromStr,
};

use crate::{common::SAML2Obj, error::SAMLError, xml::XmlObject};

use super::{
    authn_context_class_ref::AuthnContextClassRef,
    authn_context_comparison_type_enumeration::AuthnContextComparisonTypeEnumeration,
    authn_context_decl_ref::AuthnContextDeclRef,
};

#[derive(Default, Debug)]
pub struct RequestedAuthnContext {
    authn_context_class_refs: Vec<AuthnContextClassRef>,
    authn_context_decl_refs: Vec<AuthnContextDeclRef>,
    comparison: Option<AuthnContextComparisonTypeEnumeration>,
}

impl SAML2Obj for RequestedAuthnContext {}

impl RequestedAuthnContext {
    const CHILD_AUTHN_CONTEXT_CLASS_REF: &'static str = "AuthnContextClassRef";
    const CHILD_AUTHN_CONTEXT_DECL_REF: &'static str = "AuthnContextDeclRef";
    const ATTRIB_COMPARISON: &'static str = "Comparison";

    const ELEMENT_NAME: &'static str = "RequestedAuthnContext";
    const NS_PREFIX: &'static str = "saml2p";
    const NS_URI: &'static str = "urn:oasis:names:tc:SAML:2.0:protocol";

    #[inline]
    pub fn authn_context_class_refs(&self) -> &Vec<AuthnContextClassRef> {
        &self.authn_context_class_refs
    }

    #[inline]
    pub fn authn_context_decl_refs(&self) -> &Vec<AuthnContextDeclRef> {
        &self.authn_context_decl_refs
    }

    #[inline]
    pub fn comparison(&self) -> Option<&AuthnContextComparisonTypeEnumeration> {
        self.comparison.as_ref()
    }

    #[inline]
    pub fn add_authn_context_class_ref(&mut self, class_ref: AuthnContextClassRef) {
        self.authn_context_class_refs.push(class_ref);
    }

    #[inline]
    pub fn add_authn_context_decl_ref(&mut self, decl_ref: AuthnContextDeclRef) {
        self.authn_context_decl_refs.push(decl_ref);
    }

    #[inline]
    pub fn set_comparison(&mut self, comparison: Option<AuthnContextComparisonTypeEnumeration>) {
        self.comparison = comparison;
    }
}

impl TryFrom<Ref<'_, XmlObject>> for RequestedAuthnContext {
    type Error = SAMLError;

    fn try_from(element: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        let mut requested_authn_context = RequestedAuthnContext::default();
        for attrib in element.attributes() {
            match attrib.0.as_str() {
                Self::ATTRIB_COMPARISON => {
                    requested_authn_context.set_comparison(Some(
                        AuthnContextComparisonTypeEnumeration::from_str(attrib.1.as_str())?,
                    ));
                }
                _ => {}
            }
        }
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

impl TryFrom<RequestedAuthnContext> for XmlObject {
    type Error = SAMLError;

    fn try_from(requested_authn_context: RequestedAuthnContext) -> Result<Self, Self::Error> {
        let mut xml_object = XmlObject::new(
            Some(RequestedAuthnContext::NS_PREFIX.to_string()),
            RequestedAuthnContext::ELEMENT_NAME.to_string(),
            Some(RequestedAuthnContext::NS_URI.to_string()),
        );
        xml_object.add_namespace(
            RequestedAuthnContext::NS_PREFIX.to_string(),
            RequestedAuthnContext::NS_URI.to_string(),
        );
        for class_ref in requested_authn_context.authn_context_class_refs {
            xml_object.add_child(Rc::new(RefCell::new(XmlObject::try_from(class_ref)?)));
        }
        for decl_ref in requested_authn_context.authn_context_decl_refs {
            xml_object.add_child(Rc::new(RefCell::new(XmlObject::try_from(decl_ref)?)));
        }
        if let Some(comparison) = requested_authn_context.comparison {
            xml_object.add_attribute(
                RequestedAuthnContext::ATTRIB_COMPARISON.to_string(),
                comparison.to_string(),
            );
        }
        Ok(xml_object)
    }
}
