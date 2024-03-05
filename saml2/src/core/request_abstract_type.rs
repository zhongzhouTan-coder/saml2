use std::any::Any;

use chrono::{DateTime, Utc};

use super::{extensions::Extensions, issuer::Issuer, saml_version::SAMLVersion};

pub trait RequestAbstractType: Any {
    fn version(&self) -> &SAMLVersion;

    fn set_version(&mut self, saml_version: SAMLVersion);

    fn id(&self) -> &String;

    fn set_id(&mut self, id: String);

    fn issue_instant(&self) -> &DateTime<Utc>;

    fn set_issue_instant(&mut self, issue_instant: DateTime<Utc>);

    fn destination(&self) -> Option<&String>;

    fn set_destination(&mut self, destination: Option<String>);

    fn consent(&self) -> Option<&String>;

    fn set_consent(&mut self, consent: Option<String>);

    fn issuer(&self) -> Option<&Issuer>;

    fn set_issuer(&mut self, issuer: Option<Issuer>);

    fn extensions(&self) -> Option<&Extensions>;

    fn set_extensions(&mut self, extensions: Option<Extensions>);

    fn signature(&self) -> Option<&String>;

    fn set_signature(&mut self, signature: Option<String>);

    fn as_any(&self) -> &dyn Any;
}
