use chrono::{DateTime, Utc};

#[derive(Clone, Debug)]
pub struct SubjectConfirmationData {
    not_before: Option<DateTime<Utc>>,
    not_on_or_after: Option<DateTime<Utc>>,
    recipient: Option<String>,
    in_response_to: Option<String>,
    address: Option<String>,
}
