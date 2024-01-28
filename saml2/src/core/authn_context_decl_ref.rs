#[derive(Clone)]
pub struct AuthnContextDeclRef {
    value: Option<String>,
}

impl AuthnContextDeclRef {
    pub fn new(value: Option<String>) -> AuthnContextDeclRef {
        AuthnContextDeclRef { value }
    }

    pub fn value(&self) -> Option<&String> {
        self.value.as_ref()
    }

    pub fn set_value(&mut self, value: Option<String>) {
        self.value = value;
    }
}
