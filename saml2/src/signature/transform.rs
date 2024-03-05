use crate::common::SAML2Obj;

#[derive(Default, Debug)]
pub struct Transform {
    algorithm: String,
    indexed_children: Vec<Box<dyn SAML2Obj>>,
}

impl SAML2Obj for Transform {}
