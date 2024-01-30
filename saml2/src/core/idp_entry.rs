#[derive(Clone, Default, Debug)]
pub struct IDPEntry {
    provider_id: String,
    name: Option<String>,
    loc: Option<String>,
}
