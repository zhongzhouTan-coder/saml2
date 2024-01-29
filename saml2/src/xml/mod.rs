use std::{
    cell::RefCell,
    fmt,
    io::Read,
    rc::{Rc, Weak},
};

use xml::{reader::XmlEvent, EventReader};

use self::q_name::QName;

mod q_name;

pub struct XmlObject {
    parent: Option<Weak<RefCell<XmlObject>>>,
    children: Vec<Rc<RefCell<XmlObject>>>,
    attributes: Vec<(String, String)>,
    q_name: QName,
    text: Option<String>,
}

pub struct XmlError {
    message: String,
}

impl XmlError {
    pub fn new(message: String) -> Self {
        XmlError { message }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for XmlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl fmt::Debug for XmlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl XmlObject {
    pub fn new(namespace_uri: Option<String>, local_name: String, prefix: Option<String>) -> Self {
        Self {
            parent: None,
            children: Vec::new(),
            attributes: Vec::new(),
            q_name: QName::new(namespace_uri, local_name, prefix),
            text: None,
        }
    }

    pub fn parse_xml<I: Read>(mut input: I) -> Result<Rc<RefCell<XmlObject>>, XmlError> {
        let reader = EventReader::new(input);
        let mut xml_objects: Vec<Rc<RefCell<XmlObject>>> = Vec::new();
        for e in reader {
            match e {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    let mut object = XmlObject::new(
                        name.namespace.clone(),
                        name.local_name.clone(),
                        name.prefix.clone(),
                    );
                    object.attributes = attributes
                        .iter()
                        .map(|a| (a.name.local_name.clone(), a.value.clone()))
                        .collect();
                    if let Some(parent) = xml_objects.last_mut() {
                        object.parent = Some(Rc::downgrade(parent));
                    }
                    xml_objects.push(Rc::new(RefCell::new(object)));
                }
                Ok(XmlEvent::EndElement { .. }) => {
                    if xml_objects.is_empty() {
                        return Err(XmlError {
                            message: String::from("invalid xml document"),
                        });
                    }
                    let child = xml_objects.pop().unwrap();
                    if let Some(parent) = xml_objects.last_mut() {
                        parent.borrow_mut().children.push(child);
                    }
                }
                Ok(XmlEvent::Characters(s)) => {
                    if xml_objects.is_empty() {
                        return Err(XmlError {
                            message: String::from("invalid xml document"),
                        });
                    }
                    let parent = xml_objects.last_mut().unwrap();
                    parent.borrow_mut().text = Some(s);
                }
                _ => {}
            }
        }
        xml_objects.pop().ok_or(XmlError {
            message: String::from("invalid xml document"),
        })
    }
}

impl fmt::Display for XmlObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "QName: {}", self.q_name)?;
        if let Some(ref text) = self.text {
            write!(f, ", Text: {}", text)?;
        };
        for attribute in self.attributes.iter() {
            write!(f, ", Attribute: {} -> {}", attribute.0, attribute.1)?;
        }
        for child in self.children.iter() {
            write!(f, ", Child: {}", child.borrow())?;
        }
        Ok(())
    }
}
