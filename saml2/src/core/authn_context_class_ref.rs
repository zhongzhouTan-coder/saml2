#[derive(Clone)]
pub struct AuthnContextClassRef {
    value: Option<String>,
}

impl AuthnContextClassRef {
    pub fn new(value: Option<String>) -> AuthnContextClassRef {
        AuthnContextClassRef { value }
    }

    pub fn value(&self) -> Option<&String> {
        self.value.as_ref()
    }

    pub fn set_value(&mut self, value: Option<String>) {
        self.value = value;
    }
}
