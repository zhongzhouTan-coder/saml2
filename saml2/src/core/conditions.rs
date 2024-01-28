use std::time::Instant;
#[derive(Clone)]
pub struct Conditions {
    not_before: Option<Instant>,
    not_on_or_after: Option<Instant>,
    condition: String,
    audience_restriction: String,
    one_time_use: String,
    proxy_restriction: String,
}

impl Conditions {
    pub fn not_before(&self) -> Option<Instant> {
        self.not_before
    }

    pub fn set_not_before(&mut self, not_before: Option<Instant>) {
        self.not_before = not_before
    }

    pub fn not_on_or_after(&self) -> Option<Instant> {
        self.not_on_or_after
    }

    pub fn set_not_on_or_after(&mut self, not_on_or_after: Option<Instant>) {
        self.not_on_or_after = not_on_or_after
    }

    pub fn condition(&self) -> &String {
        &self.condition
    }

    pub fn set_condition(&mut self, condition: String) {
        self.condition = condition
    }

    pub fn audience_restriction(&self) -> &String {
        &self.audience_restriction
    }

    pub fn set_audience_restriction(&mut self, audience_restriction: String) {
        self.audience_restriction = audience_restriction
    }

    pub fn one_time_use(&self) -> &String {
        &self.one_time_use
    }

    pub fn set_one_time_use(&mut self, one_time_use: String) {
        self.one_time_use = one_time_use
    }

    pub fn proxy_restriction(&self) -> &String {
        &self.proxy_restriction
    }

    pub fn set_proxy_restriction(&mut self, proxy_restriction: String) {
        self.proxy_restriction = proxy_restriction
    }
}
