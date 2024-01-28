use std::collections::HashMap;

use base64::{prelude::BASE64_STANDARD, Engine};
use inflate::inflate_bytes;
use xml::{reader::XmlEvent, EventReader};

use crate::util::InputStream;

pub fn decode(params: &HashMap<String, String>) {
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
        let parser = EventReader::new(InputStream::new(saml_message));
        let mut depth = 0;
        for e in parser {
            match e {
                Ok(XmlEvent::StartElement { name, .. }) => {
                    println!("{:spaces$}+{name}", "", spaces = depth * 2);
                    depth += 1;
                }
                Ok(XmlEvent::EndElement { name }) => {
                    depth -= 1;
                    println!("{:spaces$}-{name}", "", spaces = depth * 2);
                }
                Err(e) => {
                    eprintln!("Error: {e}");
                    break;
                }
                // There's more: https://docs.rs/xml-rs/latest/xml/reader/enum.XmlEvent.html
                _ => {}
            }
        }
    } else {
        // todo throw error
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::decode;

    #[test]
    fn we_can_decode_base64_encode_inflate_saml_message() {
        let messages = "jVJdj9owEPwrkd8dJyG5CAuQ6NEPJArooH3oC/I5y2E1sV3v5j7662vCnXqV2lPfrPXO7MzsTlB1rZfznk72Bn70gJQ8dq1FOXxMWR+sdAoNSqs6QEla7uafV7JIM+mDI6ddy15B3kYoRAhknGXJcjFlm/X71ebjcn2orsp6XEHGs1FR8DK7avi40iOeN/mxruqRqqFmyVcIGLFTFqkiAWIPS4ukLMVSVpQ8y3lR7fNK5qUsx99Y8sEFDYO5KaPQA0sW0aGxigaeE5FHKYQh+skJ9In7VtHRhS7VNsV7nWoXvLEUVGqBhPJGqMgmzk4LgehYsn0O4Z2xjbF3b/u/vTSh/LTfb/l2s9uzZP6SybWz2HcQdhDujYYvN6vfCr+bW2UVVw/I7ePfxR18298Z+9wqDs5DHIYUnDgg6D4YehqEC6WRzSbnpxxCDLP/HzMRr3GTy/mso9HlYutao5/OmXeK/p1DnuZDxTT8OLTK3qIHbY4GmhhH27qH6wCK4GVnYnaZ+uedzn4B";
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert("SAMLRequest".to_string(), messages.to_string());
        decode(&params);
    }
}
