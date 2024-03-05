use std::fmt::Display;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct QName {
    namespace_uri: Option<String>,
    local_name: String,
    prefix: Option<String>,
}

impl QName {
    pub fn new(namespace_uri: Option<String>, local_name: String, prefix: Option<String>) -> QName {
        QName {
            namespace_uri,
            local_name,
            prefix,
        }
    }

    #[inline]
    pub fn namespace_uri(&self) -> Option<&str> {
        self.namespace_uri.as_ref().map(|s| s.as_str())
    }

    #[inline]
    pub fn local_name(&self) -> &str {
        &self.local_name
    }

    #[inline]
    pub fn prefix(&self) -> Option<&str> {
        self.prefix.as_ref().map(|s| s.as_str())
    }

    #[inline]
    pub fn set_prefix(&mut self, prefix: Option<String>) {
        self.prefix = prefix;
    }

    #[inline]
    pub fn set_namespace_uri(&mut self, namespace_uri: Option<String>) {
        self.namespace_uri = namespace_uri;
    }

    #[inline]
    pub fn set_local_name(&mut self, local_name: String) {
        self.local_name = local_name;
    }
}

impl Display for QName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.prefix() {
            Some(prefix) => write!(f, "{}:{}", prefix, self.local_name()),
            None => write!(f, "{}", self.local_name),
        }
    }
}
