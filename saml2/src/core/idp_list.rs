use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use crate::{common::SAML2Obj, error::SAMLError, xml::XmlObject};

use super::{get_complete::GetComplete, idp_entry::IDPEntry};

#[derive(Debug, Default)]
pub struct IDPList {
    idp_entry: Vec<IDPEntry>,
    get_complete: Option<GetComplete>,
}

impl SAML2Obj for IDPList {}

impl IDPList {
    const ELEMENT_NAME: &'static str = "IDPList";
    const NS_PREFIX: &'static str = "saml2";
    const NS_URI: &'static str = "urn:oasis:names:tc:SAML:2.0:protocol";

    const CHILD_IDP_ENTRY: &'static str = "IDPEntry";
    const CHILD_GET_COMPLETE: &'static str = "GetComplete";

    #[inline]
    pub fn idp_entry(&self) -> &Vec<IDPEntry> {
        &self.idp_entry
    }

    #[inline]
    pub fn get_complete(&self) -> Option<&GetComplete> {
        self.get_complete.as_ref()
    }

    #[inline]
    pub fn set_idp_entry(&mut self, idp_entry: Vec<IDPEntry>) {
        self.idp_entry = idp_entry;
    }

    #[inline]
    pub fn set_get_complete(&mut self, get_complete: Option<GetComplete>) {
        self.get_complete = get_complete;
    }
}

impl TryFrom<Ref<'_, XmlObject>> for IDPList {
    type Error = SAMLError;

    fn try_from(element: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        let mut idp_list = IDPList::default();
        for child in element.children() {
            let child = child.borrow();
            match child.q_name().local_name() {
                IDPList::CHILD_IDP_ENTRY => {
                    idp_list.idp_entry.push(IDPEntry::try_from(child)?);
                }
                IDPList::CHILD_GET_COMPLETE => {
                    idp_list.get_complete = Some(GetComplete::try_from(child)?);
                }
                _ => {}
            }
        }
        Ok(idp_list)
    }
}

impl TryFrom<IDPList> for XmlObject {
    type Error = SAMLError;

    fn try_from(idp_list: IDPList) -> Result<Self, Self::Error> {
        let mut xml_object = XmlObject::new(
            Some(IDPList::NS_PREFIX.to_string()),
            IDPList::ELEMENT_NAME.to_string(),
            Some(IDPList::NS_URI.to_string()),
        );
        xml_object.add_namespace(IDPList::NS_PREFIX.to_string(), IDPList::NS_URI.to_string());
        for idp_entry in idp_list.idp_entry {
            xml_object.add_child(Rc::new(RefCell::new(XmlObject::try_from(idp_entry)?)));
        }
        if let Some(get_complete) = idp_list.get_complete {
            xml_object.add_child(Rc::new(RefCell::new(XmlObject::try_from(get_complete)?)));
        }
        Ok(xml_object)
    }
}
