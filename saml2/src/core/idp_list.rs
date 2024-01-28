use super::idp_entry::IDPEntry;

#[derive(Clone)]
pub struct IDPList {
    idp_entry: Vec<IDPEntry>,
    get_complete: Option<String>,
}
