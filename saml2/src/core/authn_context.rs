use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use crate::{common::SAML2Obj, error::SAMLError, xml::XmlObject};

use super::{
    authenticating_authority::AuthenticatingAuthority,
    authn_context_class_ref::AuthnContextClassRef, authn_context_decl::AuthnContextDecl,
    authn_context_decl_ref::AuthnContextDeclRef,
};

#[derive(Debug, Default)]
pub struct AuthnContext {
    authn_context_class_ref: Option<AuthnContextClassRef>,
    authn_context_decl: Option<AuthnContextDecl>,
    authn_context_decl_ref: Option<AuthnContextDeclRef>,
    authenticating_authorities: Vec<AuthenticatingAuthority>,
}

impl SAML2Obj for AuthnContext {}

impl AuthnContext {
    const ELEMENT_NAME: &'static str = "AuthnContext";
    const NS_PREFIX: &'static str = "saml2";
    const NS_URI: &'static str = "urn:oasis:names:tc:SAML:2.0:assertion";

    const CHILD_AUTHN_CONTEXT_CLASS_REF: &'static str = "AuthnContextClassRef";
    const CHILD_AUTHN_CONTEXT_DECL: &'static str = "AuthnContextDecl";
    const CHILD_AUTHN_CONTEXT_DECL_REF: &'static str = "AuthnContextDeclRef";
    const CHILD_AUTHENTICATING_AUTHORITY: &'static str = "AuthenticatingAuthority";

    #[inline]
    pub fn authn_context_class_ref(&self) -> Option<&AuthnContextClassRef> {
        self.authn_context_class_ref.as_ref()
    }

    #[inline]
    pub fn authn_context_decl(&self) -> Option<&AuthnContextDecl> {
        self.authn_context_decl.as_ref()
    }

    #[inline]
    pub fn authn_context_decl_ref(&self) -> Option<&AuthnContextDeclRef> {
        self.authn_context_decl_ref.as_ref()
    }

    #[inline]
    pub fn authenticating_authorities(&self) -> &Vec<AuthenticatingAuthority> {
        &self.authenticating_authorities
    }

    #[inline]
    pub fn set_authn_context_class_ref(
        &mut self,
        authn_context_class_ref: Option<AuthnContextClassRef>,
    ) {
        self.authn_context_class_ref = authn_context_class_ref;
    }

    #[inline]
    pub fn set_authn_context_decl(&mut self, authn_context_decl: Option<AuthnContextDecl>) {
        self.authn_context_decl = authn_context_decl;
    }

    #[inline]
    pub fn set_authn_context_decl_ref(
        &mut self,
        authn_context_decl_ref: Option<AuthnContextDeclRef>,
    ) {
        self.authn_context_decl_ref = authn_context_decl_ref;
    }

    #[inline]
    pub fn add_authenticating_authority(
        &mut self,
        authenticating_authority: AuthenticatingAuthority,
    ) {
        self.authenticating_authorities
            .push(authenticating_authority);
    }
}

impl TryFrom<Ref<'_, XmlObject>> for AuthnContext {
    type Error = SAMLError;

    fn try_from(object: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        let mut authn_context = AuthnContext::default();
        for child in object.children() {
            let child = child.borrow();
            match child.q_name().local_name() {
                AuthnContext::CHILD_AUTHN_CONTEXT_CLASS_REF => {
                    authn_context
                        .set_authn_context_class_ref(Some(AuthnContextClassRef::try_from(child)?));
                }
                AuthnContext::CHILD_AUTHN_CONTEXT_DECL => {
                    authn_context.set_authn_context_decl(Some(AuthnContextDecl::try_from(child)?));
                }
                AuthnContext::CHILD_AUTHN_CONTEXT_DECL_REF => {
                    authn_context
                        .set_authn_context_decl_ref(Some(AuthnContextDeclRef::try_from(child)?));
                }
                AuthnContext::CHILD_AUTHENTICATING_AUTHORITY => {
                    authn_context
                        .add_authenticating_authority(AuthenticatingAuthority::try_from(child)?);
                }
                _ => {}
            }
        }
        Ok(authn_context)
    }
}

impl TryFrom<AuthnContext> for XmlObject {
    type Error = SAMLError;

    fn try_from(authn_context: AuthnContext) -> Result<Self, Self::Error> {
        let mut xml_object = XmlObject::new(
            Some(AuthnContext::NS_PREFIX.to_string()),
            AuthnContext::ELEMENT_NAME.to_string(),
            Some(AuthnContext::NS_URI.to_string()),
        );
        xml_object.add_namespace(
            AuthnContext::NS_PREFIX.to_string(),
            AuthnContext::NS_URI.to_string(),
        );
        if let Some(authn_context_class_ref) = authn_context.authn_context_class_ref {
            xml_object.add_child(Rc::new(RefCell::new(XmlObject::try_from(
                authn_context_class_ref,
            )?)));
        }
        if let Some(authn_context_decl) = authn_context.authn_context_decl {
            xml_object.add_child(Rc::new(RefCell::new(XmlObject::try_from(
                authn_context_decl,
            )?)));
        }
        if let Some(authn_context_decl_ref) = authn_context.authn_context_decl_ref {
            xml_object.add_child(Rc::new(RefCell::new(XmlObject::try_from(
                authn_context_decl_ref,
            )?)));
        }
        for authenticating_authority in authn_context.authenticating_authorities {
            xml_object.add_child(Rc::new(RefCell::new(XmlObject::try_from(
                authenticating_authority,
            )?)));
        }
        Ok(xml_object)
    }
}
