use std::cell::Ref;

use crate::{common::SAML2Obj, error::SAMLError, util::AttributeMap, xml::XmlObject};

#[derive(Debug, Default)]
pub struct AuthnContextDecl {
    text_content: Option<String>,
    unknown_xml_objects: Vec<Box<dyn SAML2Obj>>,
    unknown_attributes: AttributeMap,
}

impl SAML2Obj for AuthnContextDecl {}

impl AuthnContextDecl {
    const ELEMENT_NAME: &'static str = "AuthnContextDecl";
    const NS_PREFIX: &'static str = "saml";
    const NS_URI: &'static str = "urn:oasis:names:tc:SAML:2.0:assertion";

    #[inline]
    pub fn text_content(&self) -> Option<&String> {
        self.text_content.as_ref()
    }

    #[inline]
    pub fn set_text_content(&mut self, text_content: Option<String>) {
        self.text_content = text_content;
    }

    #[inline]
    pub fn unknown_xml_objects(&self) -> &Vec<Box<dyn SAML2Obj>> {
        &self.unknown_xml_objects
    }

    #[inline]
    pub fn unknown_attributes(&self) -> &AttributeMap {
        &self.unknown_attributes
    }
}

impl TryFrom<Ref<'_, XmlObject>> for AuthnContextDecl {
    type Error = SAMLError;

    fn try_from(object: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        let mut authn_context_decl = AuthnContextDecl::default();
        for attrib in object.attributes() {
            todo!("Handle attribute: {:?}", attrib);
        }
        // TODO: Handle unknown XML objects
        authn_context_decl.set_text_content(object.text().map(|s| s.to_string()));
        Ok(authn_context_decl)
    }
}

impl TryFrom<AuthnContextDecl> for XmlObject {
    type Error = SAMLError;

    fn try_from(authn_context_decl: AuthnContextDecl) -> Result<XmlObject, Self::Error> {
        let mut xml_object = XmlObject::new(
            Some(AuthnContextDecl::NS_PREFIX.to_string()),
            AuthnContextDecl::ELEMENT_NAME.to_string(),
            Some(AuthnContextDecl::NS_URI.to_string()),
        );
        xml_object.add_namespace(
            AuthnContextDecl::NS_PREFIX.to_string(),
            AuthnContextDecl::NS_URI.to_string(),
        );
        if let Some(text_content) = authn_context_decl.text_content() {
            xml_object.set_text(Some(text_content.to_string()));
        }
        // TODO: Add unknown attributes and unknown XML objects
        Ok(xml_object)
    }
}
