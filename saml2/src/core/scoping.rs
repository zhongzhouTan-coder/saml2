use super::{idp_list::IDPList, requester_id::RequesterID};

#[derive(Clone)]
pub struct Scoping {
    proxy_count: Option<usize>,
    idp_list: Option<IDPList>,
    requester_ids: Option<RequesterID>,
}

impl Scoping {
    pub fn proxy_count(&self) -> Option<usize> {
        self.proxy_count
    }

    pub fn set_proxy_count(&mut self, value: Option<usize>) {
        self.proxy_count = value;
    }

    pub fn idp_list(&self) -> Option<&IDPList> {
        self.idp_list.as_ref()
    }

    pub fn set_idp_list(&mut self, value: Option<IDPList>) {
        self.idp_list = value;
    }

    pub fn requester_ids(&self) -> Option<&RequesterID> {
        self.requester_ids.as_ref()
    }

    pub fn set_requester_ids(&mut self, value: Option<RequesterID>) {
        self.requester_ids = value;
    }
}
