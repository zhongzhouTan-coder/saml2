#[derive(Clone, Debug)]
pub struct BaseID {
    name_qualifier: Option<String>,
    sp_name_qualifier: Option<String>,
}

impl BaseID {
    pub fn new(name_qualifier: Option<String>, sp_name_qualifier: Option<String>) -> Self {
        BaseID {
            name_qualifier,
            sp_name_qualifier,
        }
    }

    pub fn name_qualifier(&self) -> Option<&String> {
        self.name_qualifier.as_ref()
    }

    pub fn set_name_qualifier(&mut self, name_qualifier: Option<String>) {
        self.name_qualifier = name_qualifier;
    }

    pub fn sp_name_qualifier(&self) -> Option<&String> {
        self.sp_name_qualifier.as_ref()
    }

    pub fn set_sp_name_qualifier(&mut self, sp_name_qualifier: Option<String>) {
        self.sp_name_qualifier = sp_name_qualifier;
    }
}
