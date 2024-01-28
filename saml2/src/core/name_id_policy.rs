#[derive(Clone)]
pub struct NameIDPolicy {
    format: Option<String>,
    sp_name_qualifier: Option<String>,
    allows_create: Option<String>,
}

impl NameIDPolicy {
    pub fn format(&self) -> Option<&String> {
        self.format.as_ref()
    }

    pub fn set_format(&mut self, format: Option<String>) {
        self.format = format
    }

    pub fn sp_name_qualifier(&self) -> Option<&String> {
        self.sp_name_qualifier.as_ref()
    }

    pub fn set_sp_name_qualifier(&mut self, sp_name_qualifier: Option<String>) {
        self.sp_name_qualifier = sp_name_qualifier
    }

    pub fn allows_create(&self) -> Option<&String> {
        self.allows_create.as_ref()
    }

    pub fn set_allows_create(&mut self, allows_create: Option<String>) {
        self.allows_create = allows_create
    }
}
