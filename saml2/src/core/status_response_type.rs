use chrono::{DateTime, Utc};
use crate::core::extensions::Extensions;
use crate::core::issuer::Issuer;
use crate::core::saml_version::SAMLVersion;
use crate::core::status::Status;

pub trait StatusResponseType {
    fn id(&self) -> &String;

    fn set_id(&mut self, id: String);

    fn in_response_to(&self) -> Option<&String>;

    fn set_in_response_to(&mut self, in_response_to: Option<String>);

    fn version(&self) -> &SAMLVersion;

    fn set_version(&mut self, version: SAMLVersion);

    fn issue_instant(&self) -> DateTime<Utc>;

    fn set_issue_instant(&mut self, issue_instant: DateTime<Utc>);

    fn destination(&self) -> Option<&String>;

    fn set_destination(&mut self, destination: Option<String>);

    fn consent(&self) -> Option<&String>;

    fn set_consent(&mut self, consent: Option<String>);

    fn issuer(&self) -> Option<&Issuer>;

    fn set_issuer(&mut self, issuer: Option<Issuer>);

    fn signature(&self) -> Option<&String>;

    fn set_signature(&mut self, signature: Option<String>);

    fn extensions(&self) -> Option<&Extensions>;

    fn set_extensions(&mut self, extensions: Option<Extensions>);

    fn status(&self) -> &Status;

    fn set_status(&mut self, status: Status);
}
