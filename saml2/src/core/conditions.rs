use std::{cell::Ref, str::FromStr};

use chrono::{DateTime, Utc};

use crate::{error::SAMLError, xml::XmlObject};

use super::audience_restriction::AudienceRestriction;

#[derive(Debug)]
pub struct Conditions {
    not_before: Option<DateTime<Utc>>,
    not_on_or_after: Option<DateTime<Utc>>,
    condition: String,
    audience_restriction: AudienceRestriction,
    one_time_use: String,
    proxy_restriction: String,
}

impl Conditions {
    const ATTRIB_NOT_BEFORE: &'static str = "NotBefore";
    const ATTRIB_NOT_ON_OR_AFTER: &'static str = "NotOnOrAfter";
    const ATTRIB_CONDITION: &'static str = "Condition";
    const ATTRIB_ONE_TIME_USE: &'static str = "OneTimeUse";
    const ATTRIB_PROXY_RESTRICTION: &'static str = "ProxyRestriction";

    const CHILD_AUDIENCE_RESTRICTION: &'static str = "AudienceRestriction";

    pub fn new() -> Self {
        Self {
            not_before: Default::default(),
            not_on_or_after: Default::default(),
            condition: Default::default(),
            audience_restriction: Default::default(),
            one_time_use: Default::default(),
            proxy_restriction: Default::default(),
        }
    }

    pub fn not_before(&self) -> Option<DateTime<Utc>> {
        self.not_before
    }

    pub fn set_not_before(&mut self, not_before: Option<DateTime<Utc>>) {
        self.not_before = not_before
    }

    pub fn not_on_or_after(&self) -> Option<DateTime<Utc>> {
        self.not_on_or_after
    }

    pub fn set_not_on_or_after(&mut self, not_on_or_after: Option<DateTime<Utc>>) {
        self.not_on_or_after = not_on_or_after
    }

    pub fn condition(&self) -> &String {
        &self.condition
    }

    pub fn set_condition(&mut self, condition: String) {
        self.condition = condition
    }

    pub fn audience_restriction(&self) -> &AudienceRestriction {
        &self.audience_restriction
    }

    pub fn set_audience_restriction(&mut self, audience_restriction: AudienceRestriction) {
        self.audience_restriction = audience_restriction
    }

    pub fn one_time_use(&self) -> &String {
        &self.one_time_use
    }

    pub fn set_one_time_use(&mut self, one_time_use: String) {
        self.one_time_use = one_time_use
    }

    pub fn proxy_restriction(&self) -> &String {
        &self.proxy_restriction
    }

    pub fn set_proxy_restriction(&mut self, proxy_restriction: String) {
        self.proxy_restriction = proxy_restriction
    }
}

impl TryFrom<Ref<'_, XmlObject>> for Conditions {
    type Error = SAMLError;
    fn try_from(element: Ref<'_, XmlObject>) -> Result<Self, Self::Error> {
        fn parse_from_string<T: FromStr>(xml_string: &str) -> Result<T, SAMLError> {
            xml_string
                .parse::<T>()
                .map_err(|_| SAMLError::UnmarshallingError("parse value error".to_string()))
        }

        let mut conditions = Conditions::new();
        for attribute in element.attributes() {
            match attribute.0.as_str() {
                Conditions::ATTRIB_NOT_BEFORE => {
                    conditions.not_before =
                        Some(parse_from_string::<DateTime<Utc>>(attribute.1.as_str())?);
                }
                Conditions::ATTRIB_NOT_ON_OR_AFTER => {
                    conditions.not_on_or_after =
                        Some(parse_from_string::<DateTime<Utc>>(attribute.1.as_str())?);
                }
                Conditions::ATTRIB_CONDITION => {
                    conditions.condition = attribute.1.to_string();
                }
                Conditions::ATTRIB_ONE_TIME_USE => {
                    conditions.one_time_use = attribute.1.to_string();
                }
                Conditions::ATTRIB_PROXY_RESTRICTION => {
                    conditions.proxy_restriction = attribute.1.to_string();
                }
                _ => {}
            }
        }
        for child in element.children() {
            let child = child.borrow();
            match child.q_name().local_name() {
                Conditions::CHILD_AUDIENCE_RESTRICTION => {
                    conditions.audience_restriction = AudienceRestriction::try_from(child)?;
                }
                _ => {}
            }
        }
        Ok(conditions)
    }
}
