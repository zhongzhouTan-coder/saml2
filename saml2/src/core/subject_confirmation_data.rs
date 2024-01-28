use std::time::Instant;

#[derive(Clone)]
pub struct SubjectConfirmationData {
    not_before: Option<Instant>,
    not_on_or_after: Option<Instant>,
    recipient: Option<String>,
    in_response_to: Option<String>,
    address: Option<String>,
}
