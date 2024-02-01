use std::collections::HashMap;

use base64::{prelude::BASE64_STANDARD, Engine};
use inflate::inflate_bytes;

use crate::core::authn_request::AuthnRequest;
use crate::core::request_abstract_type::RequestAbstractType;
use crate::core::response::Response;
use crate::core::status_response_type::StatusResponseType;
use crate::{error::SAMLError, util::InputStream, xml::XmlObject};

pub fn decode_request(
    params: &HashMap<String, String>,
) -> Result<Box<dyn RequestAbstractType>, SAMLError> {
    if let Some(saml_encoding) = params.get("SAMLEncoding") {
        if saml_encoding.trim() != "urn:oasis:names:to:SAML:2.0:bindings:URL-Encoding:DEFLATE" {
            // todo throw error
        }
    }
    if let Some(saml_message_encoded) = params
        .get("SAMLRequest")
        .or_else(|| params.get("SAMLResponse"))
    {
        let saml_message =
            inflate_bytes(&BASE64_STANDARD.decode(saml_message_encoded).unwrap()).unwrap();
        println!(
            "saml message is {:?}",
            String::from_utf8(saml_message.clone())
        );
        match XmlObject::parse_xml(InputStream::new(saml_message)) {
            Ok(xml_object) => {
                let authn_request = AuthnRequest::try_from(xml_object.borrow())?;
                Ok(Box::new(authn_request))
            }
            Err(_) => Err(SAMLError::MessageDecodingError(
                "invalid xml format!".to_string(),
            )),
        }
    } else {
        Err(SAMLError::MessageDecodingError(
            "saml message cannot be null!".to_string(),
        ))
    }
}

pub fn decode_response(
    params: &HashMap<String, String>,
) -> Result<Box<dyn StatusResponseType>, SAMLError> {
    if let Some(saml_encoding) = params.get("SAMLEncoding") {
        if saml_encoding.trim() != "urn:oasis:names:to:SAML:2.0:bindings:URL-Encoding:DEFLATE" {
            // todo throw error
        }
    }
    if let Some(saml_message_encoded) = params
        .get("SAMLRequest")
        .or_else(|| params.get("SAMLResponse"))
    {
        let saml_message =
            inflate_bytes(&BASE64_STANDARD.decode(saml_message_encoded).unwrap()).unwrap();
        println!(
            "saml message is {:?}",
            String::from_utf8(saml_message.clone())
        );
        match XmlObject::parse_xml(InputStream::new(saml_message)) {
            Ok(xml_object) => {
                let response = Response::try_from(xml_object.borrow())?;
                println!("response is {:?}", response);
                Ok(Box::new(response))
            }
            Err(_) => Err(SAMLError::MessageDecodingError(
                "invalid xml format!".to_string(),
            )),
        }
    } else {
        Err(SAMLError::MessageDecodingError(
            "saml message cannot be null!".to_string(),
        ))
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::{decode_request, decode_response};

    #[test]
    fn we_can_decode_base64_encode_inflate_saml_message() {
        let messages = "";
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert("SAMLRequest".to_string(), messages.to_string());
        let result = decode_request(&params);
        assert!(result.is_ok());
    }

    #[test]
    fn we_can_decode_base64_encode_inflate_saml_resposne() {
        let message = "";
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert("SAMLResponse".to_string(), message.to_string());
        let result = decode_response(&params);
        assert!(result.is_ok());
    }
}
