use std::fmt::Display;

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

    pub fn namespace_uri(&self) -> Option<&String> {
        self.namespace_uri.as_ref()
    }

    pub fn local_name(&self) -> &str {
        &self.local_name
    }

    pub fn prefix(&self) -> Option<&String> {
        self.prefix.as_ref()
    }

    pub fn set_prefix(&mut self, prefix: Option<String>) {
        self.prefix = prefix;
    }

    pub fn set_namespace_uri(&mut self, namespace_uri: Option<String>) {
        self.namespace_uri = namespace_uri;
    }

    pub fn set_local_name(&mut self, local_name: String) {
        self.local_name = local_name;
    }
}

impl Display for QName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.local_name)?;
        if let Some(ref prefix) = self.prefix {
            write!(f, ":{}", prefix)?;
        }
        if let Some(ref namespace_uri) = self.namespace_uri {
            write!(f, " {}\n", namespace_uri)?;
        }
        Ok(())
    }
}
