use std::{
    cell::RefCell,
    collections::BTreeMap,
    fmt,
    io::Read,
    rc::{Rc, Weak},
};

use xml::{reader::XmlEvent, EventReader, EventWriter};

use self::q_name::QName;

mod q_name;

pub struct XmlObject {
    children: Vec<Rc<RefCell<XmlObject>>>,
    attributes: Vec<(String, String)>,
    q_name: QName,
    namespace: BTreeMap<String, String>,
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
            children: Vec::new(),
            attributes: Vec::new(),
            namespace: BTreeMap::new(),
            q_name: QName::new(namespace_uri, local_name, prefix),
            text: None,
        }
    }

    #[inline]
    pub fn q_name(&self) -> &QName {
        &self.q_name
    }

    #[inline]
    pub fn set_q_name(&mut self, q_name: QName) {
        self.q_name = q_name;
    }

    #[inline]
    pub fn text(&self) -> Option<&String> {
        self.text.as_ref()
    }

    #[inline]
    pub fn set_text(&mut self, text: Option<String>) {
        self.text = text;
    }

    #[inline]
    pub fn attributes(&self) -> &Vec<(String, String)> {
        &self.attributes
    }

    #[inline]
    pub fn add_attribute(&mut self, key: String, value: String) {
        self.attributes.push((key, value));
    }

    #[inline]
    pub fn children(&self) -> &Vec<Rc<RefCell<XmlObject>>> {
        &self.children
    }

    #[inline]
    pub fn add_child(&mut self, child: Rc<RefCell<XmlObject>>) {
        self.children.push(child);
    }

    #[inline]
    pub fn namespace(&self) -> &BTreeMap<String, String> {
        &self.namespace
    }

    #[inline]
    pub fn add_namespace(&mut self, prefix: String, uri: String) {
        self.namespace.insert(prefix, uri);
    }

    pub fn parse_xml<I: Read>(input: I) -> Result<Rc<RefCell<XmlObject>>, XmlError> {
        let reader = EventReader::new(input);
        let mut xml_objects: Vec<Rc<RefCell<XmlObject>>> = Vec::new();
        for e in reader {
            match e {
                Ok(XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => {
                    let mut object = XmlObject::new(
                        name.namespace.clone(),
                        name.local_name.clone(),
                        name.prefix.clone(),
                    );
                    for (prefix, uri) in &namespace.borrow().0 {
                        object.add_namespace(prefix.clone(), uri.clone());
                    }
                    for attribute in &attributes {
                        object.add_attribute(
                            attribute.name.local_name.clone(),
                            attribute.value.clone(),
                        );
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
                    if let Some(parent) = xml_objects.last() {
                        parent.borrow_mut().children.push(child);
                    } else {
                        return Ok(child);
                    }
                }
                Ok(XmlEvent::Characters(s)) => {
                    if xml_objects.is_empty() {
                        return Err(XmlError {
                            message: String::from("invalid xml document"),
                        });
                    }
                    let parent = xml_objects.last().unwrap();
                    parent.borrow_mut().text = Some(s);
                }
                _ => {}
            }
        }
        Err(XmlError {
            message: String::from("invalid xml document"),
        })
    }

    pub fn write_xml(root: Rc<RefCell<XmlObject>>) -> Result<String, XmlError> {
        let mut output: Vec<u8> = Vec::new();
        let mut writer = EventWriter::new(&mut output);
        let mut xml_objects: Vec<(Rc<RefCell<XmlObject>>, bool)> = Vec::new();
        xml_objects.push((Rc::clone(&root), false));
        while let Some((xml_object, is_visited)) = xml_objects.pop() {
            let borrow_object = xml_object.borrow();
            if is_visited {
                let name = xml::name::Name {
                    local_name: borrow_object.q_name().local_name(),
                    namespace: borrow_object.q_name().namespace_uri(),
                    prefix: borrow_object.q_name().prefix(),
                };
                let event = xml::writer::XmlEvent::EndElement { name: Some(name) };
                writer
                    .write(event)
                    .map_err(|_| XmlError::new(String::from("invalid xml document")))?;
            } else {
                let name = xml::name::Name {
                    local_name: borrow_object.q_name().local_name(),
                    namespace: borrow_object.q_name().namespace_uri(),
                    prefix: borrow_object.q_name().prefix(),
                };
                let mut start_element = xml::writer::XmlEvent::start_element(name);
                for ns in borrow_object.namespace() {
                    let mut is_exist = false;
                    for (xml_object, v) in &xml_objects {
                        if !v {
                            continue;
                        }
                        if xml_object
                            .borrow()
                            .namespace()
                            .get(ns.0)
                            .filter(|v| *v == ns.1)
                            .is_some()
                        {
                            is_exist = true;
                            break;
                        }
                    }
                    if !is_exist {
                        start_element = start_element.ns(ns.0.as_str(), ns.1.as_str());
                    }
                }
                for attribute in borrow_object.attributes() {
                    start_element = start_element.attr(attribute.0.as_str(), attribute.1.as_str());
                }
                writer
                    .write(xml::writer::XmlEvent::from(start_element))
                    .map_err(|_| XmlError::new(String::from("invalid xml document")))?;
                if let Some(text) = borrow_object.text() {
                    writer
                        .write(xml::writer::XmlEvent::characters(text.as_str()))
                        .map_err(|_| XmlError::new(String::from("invalid xml document")))?;
                }
                xml_objects.push((Rc::clone(&xml_object), true));
                for child in borrow_object.children() {
                    xml_objects.push((Rc::clone(child), false));
                }
            }
        }
        String::from_utf8(output).map_err(|_| XmlError {
            message: String::from("invalid xml document"),
        })
    }
}

impl fmt::Display for XmlObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "local name: {}", self.q_name().local_name())?;
        if let Some(ref prefix) = self.q_name().prefix() {
            write!(f, "prefix: {}", prefix)?;
        }
        if let Some(namespace_uri) = self.q_name().namespace_uri() {
            write!(f, "namespace uri: {}", namespace_uri)?;
        }
        writeln!(f, "")?;
        for (key, value) in self.attributes() {
            writeln!(f, "attribute: {}={}", key, value)?;
        }
        for (prefix, uri) in self.namespace() {
            writeln!(f, "namespace: {}={}", prefix, uri)?;
        }
        if let Some(ref text) = self.text() {
            writeln!(f, "text: {}", text)?;
        }
        for child in self.children() {
            writeln!(f, "{}", child.borrow())?;
        }
        Ok(())
    }
}
