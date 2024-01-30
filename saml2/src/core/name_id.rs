use super::abstract_name_id_type::AbstractNameIDType;

#[derive(Clone, Debug)]
pub struct NameID {
    name_qualifier: Option<String>,
    sp_name_qualifier: Option<String>,
    format: Option<String>,
    sp_provided_id: Option<String>,
}

impl AbstractNameIDType for NameID {
    fn name_qualifier(&self) -> Option<&String> {
        self.name_qualifier.as_ref()
    }

    fn set_name_qualifier(&mut self, name_qualifier: Option<String>) {
        self.name_qualifier = name_qualifier;
    }

    fn sp_name_qualifier(&self) -> Option<&String> {
        self.sp_name_qualifier.as_ref()
    }

    fn set_sp_name_qualifier(&mut self, sp_name_qualifier: Option<String>) {
        self.sp_name_qualifier = sp_name_qualifier;
    }

    fn format(&self) -> Option<&String> {
        self.format.as_ref()
    }

    fn set_format(&mut self, format: Option<String>) {
        self.format = format;
    }

    fn sp_provided_id(&self) -> Option<&String> {
        self.sp_provided_id.as_ref()
    }

    fn set_sp_provided_id(&mut self, sp_provided_id: Option<String>) {
        self.sp_provided_id = sp_provided_id;
    }
}
