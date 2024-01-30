#[derive(Clone, Debug)]
pub enum AuthnContextComparisonTypeEnumeration {
    EXACT(String),
    MINIMUM(String),
    MAXIMUM(String),
    BETTER(String),
}
