pub struct StatusCode {
    value: String,
    status_code: Option<Box<StatusCode>>,
}