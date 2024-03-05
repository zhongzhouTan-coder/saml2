use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use super::{idp_list::IDPList, requester_id::RequesterID};
use crate::{common::SAML2Obj, error::SAMLError, xml::XmlObject};

#[derive(Default, Debug)]
pub struct Scoping {
    proxy_count: Option<i32>,
    idp_list: Option<IDPList>,
    requester_ids: Vec<RequesterID>,
}

impl SAML2Obj for Scoping {}

impl Scoping {
    const ELEMENT_NAME: &'static str = "Scoping";
    const NS_PREFIX: &'static str = "saml2";
    const NS_URI: &'static str = "urn:oasis:names:tc:SAML:2.0:protocol";

    const ATTRIB_PROXY_COUNT: &'static str = "ProxyCount";

    const CHILD_IDP_LIST: &'static str = "IDPList";
    const CHILD_REQUESTER_ID: &'static str = "RequesterID";

    #[inline]
    pub fn proxy_count(&self) -> Option<i32> {
        self.proxy_count
    }

    #[inline]
    pub fn set_proxy_count(&mut self, value: Option<i32>) {
        self.proxy_count = value;
    }

    #[inline]
    pub fn idp_list(&self) -> Option<&IDPList> {
        self.idp_list.as_ref()
    }

    #[inline]
    pub fn set_idp_list(&mut self, value: Option<IDPList>) {
        self.idp_list = value;
    }

    #[inline]
    pub fn requester_ids(&self) -> &Vec<RequesterID> {
        &self.requester_ids
    }

    #[inline]
    pub fn add_requester_id(&mut self, value: RequesterID) {
        self.requester_ids.push(value);
    }
}

impl TryFrom<Ref<'_, XmlObject>> for Scoping {
    type Error = SAMLError;

    fn try_from(element: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        let mut scoping = Scoping::default();
        for attrib in element.attributes() {
            match attrib.0.as_str() {
                Scoping::ATTRIB_PROXY_COUNT => {
                    scoping.set_proxy_count(attrib.1.parse().ok());
                }
                _ => {}
            }
        }
        for child in element.children() {
            let child = child.borrow();
            match child.q_name().local_name() {
                Scoping::CHILD_IDP_LIST => {
                    scoping.set_idp_list(Some(IDPList::try_from(child)?));
                }
                Scoping::CHILD_REQUESTER_ID => {
                    scoping.add_requester_id(RequesterID::try_from(child)?);
                }
                _ => {}
            }
        }
        Ok(scoping)
    }
}

impl TryFrom<Scoping> for XmlObject {
    type Error = SAMLError;

    fn try_from(scoping: Scoping) -> Result<Self, Self::Error> {
        let mut xml_object = XmlObject::new(
            Some(Scoping::NS_PREFIX.to_string()),
            Scoping::ELEMENT_NAME.to_string(),
            Some(Scoping::NS_URI.to_string()),
        );
        xml_object.add_namespace(Scoping::NS_PREFIX.to_string(), Scoping::NS_URI.to_string());
        if let Some(proxy_count) = scoping.proxy_count() {
            xml_object.add_attribute(
                Scoping::ATTRIB_PROXY_COUNT.to_string(),
                proxy_count.to_string(),
            );
        }
        if let Some(idp_list) = scoping.idp_list {
            xml_object.add_child(Rc::new(RefCell::new(XmlObject::try_from(idp_list)?)));
        }
        for requester_id in scoping.requester_ids {
            xml_object.add_child(Rc::new(RefCell::new(XmlObject::try_from(requester_id)?)));
        }
        Ok(xml_object)
    }
}
