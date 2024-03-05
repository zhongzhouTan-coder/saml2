use std::fmt::Debug;

use crate::common::SAML2Obj;

#[derive(Default, Debug)]
pub struct StatusDetail {
    unknown_children: Vec<Box<dyn SAML2Obj>>,
}

impl SAML2Obj for StatusDetail {}
