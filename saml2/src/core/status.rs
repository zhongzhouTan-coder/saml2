use crate::core::status_code::StatusCode;

pub struct Status {
    status_code: StatusCode,
    status_message: Option<String>,
    status_detail: Option<String>,
}