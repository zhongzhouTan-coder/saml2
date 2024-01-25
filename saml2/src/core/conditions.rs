use std::time::Instant;

pub struct Conditions {
    not_before: Option<Instant>,
    not_on_or_after: Option<Instant>,
    condition: String,
    audience_restriction: String,
    one_time_use: String,
    proxy_restriction: String,
}
