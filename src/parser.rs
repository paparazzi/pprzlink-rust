// =========================
// Ppprzlink Messages
// =========================
extern crate xml;
use std::fs::File;
use std::io::BufReader;
use std::fmt; // Import `fmt`
use self::xml::reader::{EventReader, XmlEvent};
use self::xml::attribute::OwnedAttribute;

/// two versions of pprzlink protocol
#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum PprzProtocolVersion {
    ProtocolV1,
    // ProtocolV2,
}

impl fmt::Display for PprzProtocolVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            PprzProtocolVersion::ProtocolV1 => String::from("v1.0"),
            //PprzProtocolVersion::ProtocolV2 => String::from("v2.0"),
        };
        write!(f, "{}", s)
    }
}



/// only one version of the messages for now
#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum PprzMessageVersion {
    MessagesV1,
}

impl fmt::Display for PprzMessageVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            PprzMessageVersion::MessagesV1 => String::from("v1.0"),
        };
        write!(f, "{}", s)
    }
}

/// ID of all message classes
#[derive(Debug, Copy, PartialEq, PartialOrd)]
pub enum PprzMsgClassID {
    Telemetry,
    Datalink,
    Ground,
    Alert,
    Intermcu,
}

impl fmt::Display for PprzMsgClassID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            PprzMsgClassID::Telemetry => String::from("Telemetry"),
            PprzMsgClassID::Datalink => String::from("Datalink"),
            PprzMsgClassID::Ground => String::from("Ground"),
            PprzMsgClassID::Alert => String::from("Alert"),
            PprzMsgClassID::Intermcu => String::from("Intermcu"),
        };
        write!(f, "{}", s)
    }
}

impl Clone for PprzMsgClassID {
    fn clone(&self) -> PprzMsgClassID {
        *self
    }
}

/// Values of the each message field
#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum PprzMsgBaseType {
    Float(f32),
    FloatArr(Vec<f32>),
    Double(f64),
    DoubleArr(Vec<f64>),
    Uint8(u8),
    Uint8Arr(Vec<u8>),
    Uint16(u16),
    Uint16Arr(Vec<u16>),
    Uint32(u32),
    Uint32Arr(Vec<u32>),
    Int8(i8),
    Int8Arr(Vec<i8>),
    Int16(i16),
    Int16Arr(Vec<i16>),
    Int32(i32),
    Int32Arr(Vec<i32>),
    Char(char), // maybe u8?
    CharArr(Vec<char>),
    String(String),
}

impl fmt::Display for PprzMsgBaseType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PprzMsgBaseType::Float(v) => write!(f, "float: {}", v),
            PprzMsgBaseType::FloatArr(ref v) => write!(f, "float vec: {:?}", v),
            PprzMsgBaseType::Double(v) => write!(f, "double: {}", v),
            PprzMsgBaseType::DoubleArr(ref v) => write!(f, "double vec: {:?}", v),
            PprzMsgBaseType::Uint8(v) => write!(f, "Uint8: {}", v),
            PprzMsgBaseType::Uint8Arr(ref v) => write!(f, "Uint8Arr: {:?}", v),
            PprzMsgBaseType::Uint16(v) => write!(f, "Uint16: {}", v),
            PprzMsgBaseType::Uint16Arr(ref v) => write!(f, "Uint16Arr: {:?}", v),
            PprzMsgBaseType::Uint32(v) => write!(f, "Uint32: {}", v),
            PprzMsgBaseType::Uint32Arr(ref v) => write!(f, "Uint32Arr: {:?}", v),
            PprzMsgBaseType::Int8(v) => write!(f, "int8: {}", v),
            PprzMsgBaseType::Int8Arr(ref v) => write!(f, "int8Arr: {:?}", v),
            PprzMsgBaseType::Int16(v) => write!(f, "int16: {}", v),
            PprzMsgBaseType::Int16Arr(ref v) => write!(f, "int16Arr: {:?}", v),
            PprzMsgBaseType::Int32(v) => write!(f, "int32: {}", v),
            PprzMsgBaseType::Int32Arr(ref v) => write!(f, "int32Arr: {:?}", v),
            PprzMsgBaseType::Char(v) => write!(f, "Char: {}", v),
            PprzMsgBaseType::CharArr(ref v) => write!(f, "CharArr: {:?}", v),
            PprzMsgBaseType::String(ref v) => write!(f, "String: {}", v),
        }

    }
}


/// each field has a name and a type
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PprzField {
    name: String,
    value: PprzMsgBaseType,
}

impl fmt::Display for PprzField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "name: {}, value: {}", self.name, self.value)
    }
}


/// Pprzlink message
/// see https://wiki.paparazziuav.org/wiki/Messages_Format
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PprzMessage {
    protocol: PprzProtocolVersion,
    source: u8, // SENDER_ID
    destination: u8, // can be BROADCAST
    component: u8,
    version: PprzMessageVersion, // maybe obsolete?
    id: u8, // MSG_ID
    fields: Vec<PprzField>,
    pub name: String,
}

impl PprzMessage {
    pub fn contains(&self, query: &str) -> bool {
        for field in &self.fields {
            if field.name == query {
                return true;
            }
        }
        return false;
    }
}

// TODO: use display for printing IVY compatible regexprs ?
impl fmt::Display for PprzMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = write!(
            f,
            "MESSAGE\nid: {}, name: {},
               protocol: {}, source: {}, destinaton: {}, component: {}, version: {},
               fields:",
            self.id,
            self.name,
            self.protocol,
            self.source,
            self.destination,
            self.component,
            self.version
        );
        for field in &self.fields {
            let _ = write!(f, "{}", field);
        }
        write!(f, "\n")
    }
}



/// each class has a vector of messages
#[derive(Debug, Clone)]
pub struct PprzMsgClass {
    pub messages: Vec<PprzMessage>,
    pub id: PprzMsgClassID,
}

impl PprzMsgClass {
    pub fn contains(&self, query: u8) -> bool {
        for msg in &self.messages {
            if msg.id == query {
                return true;
            }
        }
        return false;
    }
}

impl fmt::Display for PprzMsgClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = write!(f, "MESSAGE CLASS\nid: {},\nmessages:\n", self.id);
        for msg in &self.messages {
            let _ = write!(f, "{}", msg);
        }
        write!(f, "\n")
    }
}





/// Dictionary of all available messages
#[derive(Debug)]
pub struct PprzDictionary {
    pub classes: Vec<PprzMsgClass>,
}

impl PprzDictionary {
    pub fn contains(&self, query: PprzMsgClassID) -> bool {
        for class in &self.classes {
            if class.id == query {
                return true;
            }
        }
        return false;
    }

    pub fn find_msg_by_name(&self, name: &str) -> Option<PprzMessage> {
        for class in &self.classes {
            for msg in &class.messages {
                if msg.name == name {
                    return Some(msg.clone());
                }
            }
        }
        None
    }

    pub fn get_msgs(self, msg_class_id: PprzMsgClassID) -> Option<PprzMsgClass> {
        for class in self.classes {
            if class.id == msg_class_id {
                return Some(class.clone());
            }
        }
        None
    }

    pub fn get_msg_name(&self, msg_class_id: PprzMsgClassID, msg_id: u8) -> Option<String> {
        for class in &self.classes {
            if class.id == msg_class_id {
                for msg in &class.messages {
                    if msg.id == msg_id {
                        return Some(msg.name.clone());
                    }
                }
            }
        }
        None
    }


    pub fn get_msg_fields(&self,
                          msg_class_id: PprzMsgClassID,
                          msg_name: &str)
                          -> Option<Vec<PprzField>> {
        for class in &self.classes {
            if class.id == msg_class_id {
                for msg in &class.messages {
                    if msg.name == msg_name {
                        return Some(msg.fields.clone());
                    }
                }
            }
        }
        None
    }


    pub fn get_msg_id(&self, msg_class_id: PprzMsgClassID, msg_name: &str) -> Option<u8> {
        for class in &self.classes {
            if class.id == msg_class_id {
                for msg in &class.messages {
                    if msg.name == msg_name {
                        return Some(msg.id);
                    }
                }
            }
        }
        None
    }
}

impl fmt::Display for PprzDictionary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = write!(f, "DICTIONARY: \n");
        for class in &self.classes {
            let _ = write!(f, "class {}", class);
        }
        write!(f, "\n")
    }
}






///
/// Xml parser functions
///
fn has_attribute(attributes: &Vec<OwnedAttribute>, value: &str) -> (bool, usize) {
    let mut idx = 0;
    for attr in attributes {
        if attr.name.local_name == value {
            return (true, idx);
        }
        idx += 1;
    }
    return (false, idx);
}

pub fn build_dictionary(file: File) -> PprzDictionary {
    let file = BufReader::new(file);
    let mut dictionary = PprzDictionary { classes: vec![] };
    let parser = EventReader::new(file);
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                match name.local_name.as_ref() {
                    "msg_class" => {
                        // new message class - match the ID and optionally append
                        let (name_ok, idx) = has_attribute(&attributes, "name");
                        if !name_ok {
                            panic!("No attribute 'name' present. Attributes: {:?}", attributes);
                        }

                        // pattern match with the enum
                        let class_name = match attributes[idx].value.as_ref() {
                            "telemetry" => PprzMsgClassID::Telemetry,
                            "datalink" => PprzMsgClassID::Datalink,
                            "ground" => PprzMsgClassID::Ground,
                            "alert" => PprzMsgClassID::Alert,
                            "intermcu" => PprzMsgClassID::Intermcu,
                            _ => panic!("Unknown message class"),
                        };

                        // check if we have the message class already
                        if dictionary.contains(class_name) {
                            // do nothing?
                        } else {
                            // a new message class
                            let class = PprzMsgClass {
                                messages: vec![],
                                id: class_name.clone(),
                            };
                            dictionary.classes.push(class);
                        }
                    }
                    "message" => {
                        // new message - match the ID within the class and optionally append\
                        let (name_ok, name_idx) = has_attribute(&attributes, "name");
                        let (id_ok, id_idx) = has_attribute(&attributes, "id");
                        if !name_ok {
                            panic!("No attribute 'name' present. Attributes: {:?}", attributes);
                        }
                        if !id_ok {
                            panic!("No attribute 'id' present. Attributes: {:?}", attributes);
                        }
                        let msg_name = &attributes[name_idx].value;
                        let msg_id: u8 = attributes[id_idx]
                            .value
                            .parse()
                            .expect("Invalid message id");

                        let mut last_class =
                            dictionary.classes.pop().expect("No message class found!");
                        // check if we have the message already
                        if !last_class.contains(msg_id) {
                            // new message
                            let msg = PprzMessage {
                                protocol: PprzProtocolVersion::ProtocolV1,
                                source: 0,
                                destination: 0,
                                component: 0,
                                version: PprzMessageVersion::MessagesV1,
                                fields: vec![],
                                name: msg_name.clone(),
                                id: msg_id,
                            };
                            last_class.messages.push(msg);
                        }

                        // push the messahe class back either way
                        dictionary.classes.push(last_class);

                    }
                    "field" => {
                        // new message field - check within the message for existing fields and
                        // optionally append
                        let (name_ok, name_idx) = has_attribute(&attributes, "name");
                        let (type_ok, type_idx) = has_attribute(&attributes, "type");
                        if !name_ok {
                            panic!("No attribute 'name' present. Attributes: {:?}", attributes);
                        }
                        if !type_ok {
                            panic!("No attribute 'type' present. Attributes: {:?}", attributes);
                        }
                        let field_name = &attributes[name_idx].value;

                        let field_type: PprzMsgBaseType =
                            match attributes[type_idx].value.as_ref() {
                                "float" => PprzMsgBaseType::Float(0.0),
                                "float[]" => PprzMsgBaseType::FloatArr(vec![]),
                                "double" => PprzMsgBaseType::Double(0.0),
                                "double[]" => PprzMsgBaseType::DoubleArr(vec![]),
                                "uint8" => PprzMsgBaseType::Uint8(0),
                                "uint8[]" => PprzMsgBaseType::Uint8Arr(vec![]),
                                "uint16" => PprzMsgBaseType::Uint16(0),
                                "uint16[]" => PprzMsgBaseType::Uint16Arr(vec![]),
                                "uint32" => PprzMsgBaseType::Uint32(0),
                                "uint32[]" => PprzMsgBaseType::Uint32Arr(vec![]),
                                "int8" => PprzMsgBaseType::Int8(0),
                                "int8[]" => PprzMsgBaseType::Int8Arr(vec![]),
                                "int16" => PprzMsgBaseType::Int16(0),
                                "int16[]" => PprzMsgBaseType::Int16Arr(vec![]),
                                "int32" => PprzMsgBaseType::Int32(0),
                                "int32[]" => PprzMsgBaseType::Int32Arr(vec![]),
                                "char" => PprzMsgBaseType::Char(' '),
                                "char[]" => PprzMsgBaseType::CharArr(vec![]),
                                "string" => PprzMsgBaseType::String(String::new()),
                                _ => panic!("Unknown field type"),
                            };

                        // create a new field
                        let field = PprzField {
                            name: field_name.clone(),
                            value: field_type,
                        };

                        // pop last message from the last message class
                        let mut last_class =
                            dictionary.classes.pop().expect("No message class found!");
                        let mut last_msg = last_class.messages.pop().expect("No message found!");

                        // check if we have this field already
                        if !last_msg.contains(&field.name) {
                            // append field
                            last_msg.fields.push(field);
                        }

                        // push last message back to last class
                        last_class.messages.push(last_msg);

                        // pusg last class back to the dictionary
                        dictionary.classes.push(last_class);

                    }
                    _ => {}
                }

            }
            Ok(XmlEvent::EndElement { .. }) => {
                // do nothing
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
    dictionary
}
