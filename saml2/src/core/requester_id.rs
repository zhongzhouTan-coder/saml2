#[derive(Clone, Debug)]
pub struct RequesterID {
    value: Option<String>,
}

impl RequesterID {
    pub fn new(value: String) -> RequesterID {
        RequesterID { value: Some(value) }
    }

    pub fn value(&self) -> Option<&str> {
        self.value.as_ref().map(|s| s.as_str())
    }

    pub fn set_value(&mut self, value: Option<String>) {
        self.value = value;
    }
}
