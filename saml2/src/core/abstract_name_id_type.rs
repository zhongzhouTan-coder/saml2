pub trait AbstractNameIDType {
    fn name_qualifier(&self) -> Option<&String>;

    fn set_name_qualifier(&mut self, name_qualifier: Option<String>);

    fn sp_name_qualifier(&self) -> Option<&String>;

    fn set_sp_name_qualifier(&mut self, sp_name_qualifier: Option<String>);

    fn format(&self) -> Option<&String>;

    fn set_format(&mut self, format: Option<String>);

    fn sp_provided_id(&self) -> Option<&String>;

    fn set_sp_provided_id(&mut self, sp_provided_id: Option<String>);
}
