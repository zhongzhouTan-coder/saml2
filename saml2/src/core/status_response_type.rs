use std::time::Instant;

use super::extensions::Extensions;

pub struct StatusResponseType {
    id: String,
    in_response_to: Option<String>,
    version: String,
    issue_instant: Instant,
    destination: Option<String>,
    consent: Option<String>,
    issuer: Option<String>,
    signature: Option<String>,
    extensions: Option<Extensions>,
    status: Status,
}

struct Status {
    status_code: StatusCode,
    status_message: Option<String>,
    status_detail: Option<String>,
}

struct StatusCode {
    value: String,
    status_code: Option<Box<StatusCode>>,
}
