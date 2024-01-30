#[derive(Debug)]
pub enum SAMLError {
    MessageDecodingError(String),
    UnmarshallingError(String),
}
