//use std::cmp::Ordering;
use changecase::ChangeCase;
use std::collections::HashMap;
use std::default::Default;
//use std::io::{Read, Write};
use std::io::Read;
use std::path::Path;
use std::fs::File;

use xml::reader::{EventReader, XmlEvent};

use quote::{Ident, Tokens};
//use rustfmt;

/// XML elements, from messages.dtd file
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PprzXmlElement {
    Protocol,
    MsgClass,
    Message,
    Field,
    Description,
}

/// Match string to a XML element
fn identify_element(s: &str) -> Option<PprzXmlElement> {
    use self::PprzXmlElement::*;
    match s {
        "protocol" => Some(Protocol),
        "msg_class" => Some(MsgClass),
        "message" => Some(Message),
        "field" => Some(Field),
        "description" => Some(Description),
        _ => None,
    }
}

/// Encode parent-child dependencies
fn is_valid_parent(p: Option<PprzXmlElement>, s: PprzXmlElement) -> bool {
    use self::PprzXmlElement::*;
    match s {
        MsgClass => p == Some(Protocol),
        Protocol => p == None,
        Message => p == Some(MsgClass),
        Field => p == Some(Message),
        Description => p == Some(Message),
    }
}

/// Main struct, encoding information about all message classes
/// and their messahes
#[derive(Debug, PartialEq, Clone)]
pub struct PprzProfile {
    pub msg_classes: HashMap<(u8, String), PprzMsgClass>,
}

impl PprzProfile {
    /// Simple header comment
    fn emit_comments(&self) -> Ident {
        Ident::from(format!(
            "/* This file was automatically generated, do not edit */\n"
        ))
    }

    fn emit_msgs(&self) -> Vec<Tokens> {
        self.msg_classes.values()
            .map(|d| d.emit_rust())
            .collect::<Vec<Tokens>>()
    }

    fn emit_rust(&self) -> Tokens {
        let comment = self.emit_comments();
        let msg_classes = self.emit_msgs();

        quote!{
            #comment
            #(#msg_classes)*
        }
    }

    fn emit_classes(&self) -> Vec<String> {
        self.msg_classes.values()
            .map(|d| d.name.clone())
            .collect::<Vec<String>>()
    }
}


#[derive(Debug, PartialEq, Clone, Default)]
pub struct PprzMsgClass {
    pub messages: Vec<PprzMessage>,
    pub id: u8,
    pub name: String,
}

impl PprzMsgClass {
    fn emit_comments(&self) -> Ident {
        Ident::from(format!(
            "/* This file was automatically generated, do not edit */\n"
        ))
    }

    fn emit_class_name(&self) -> Tokens {
        let name = Ident::from(format!("{}", self.name));
        quote!(#name)
    }

    fn emit_msgs(&self) -> Vec<Tokens> {
        self.messages.iter()
            .map(|d| d.emit_rust())
            .collect::<Vec<Tokens>>()
    }

    fn emit_enum_names(&self) -> Vec<Tokens> {
        self.messages.iter()
            .map(|d| d.emit_enum_name())
            .collect::<Vec<Tokens>>()
    }

    fn emit_struct_names(&self) -> Vec<Tokens> {
        self.messages.iter()
            .map(|d| d.emit_struct_name())
            .collect::<Vec<Tokens>>()
    }

    fn emit_msg_names(&self) -> Vec<Tokens> {
        self.messages.iter()
            .map(|d| d.emit_msg_name())
            .collect::<Vec<Tokens>>()
    }

    /// A list of message IDs
    fn emit_msg_ids(&self) -> Vec<Tokens> {
        self.messages
            .iter()
            .map(|msg| {
                let id = Ident::from(msg.id.to_string());
                quote!(#id)
            })
            .collect::<Vec<Tokens>>()
    }


    fn emit_rust(&self) -> Tokens {
        let comment = self.emit_comments();
        let msgs = self.emit_msgs();
        let modname = self.emit_class_name();
        let enumname = Ident::from(format!("PprzMessage{}", self.name.to_capitalized()));
        let structs = self.emit_struct_names();
        let structs_parse = structs.clone();

        let enums = self.emit_enum_names();
        let enums_parse = enums.clone();
        let enums_msg_id = enums.clone();
        let enums_msg_name = enums.clone();
        let enums_serialize = enums.clone();

        let msg_ids = self.emit_msg_ids();
        let msg_ids_parse = msg_ids.clone();
        
        let msg_names = self.emit_msg_names();

        quote!{
            #[allow(non_camel_case_types)]
            #[allow(non_snake_case)]
            pub mod #modname {

            #[derive(Clone, PartialEq, Debug)]
            #[derive(Serialize, Deserialize)]
            pub enum #enumname {
                #(#enums(#structs)),*
            }

            impl #enumname {
                /*
                pub fn parse(id: u8, payload: &[u8]) -> Option<#enumname> {
                    use self::#enumname::*;
                    match id {
                        #(#msg_ids_parse => Some(#enums_parse(#structs_parse::parse(payload))),)*
                        _ => None,
                    }
                }
                */

                pub fn message_id(&self) -> u8 {
                    use self::#enumname::*;
                    match self {
                        #(#enums_msg_id(..) => #msg_ids,)*
                    }
                }

                pub fn message_name(&self) -> String {
                    use self::#enumname::*;
                    match self {
                        #(#enums_msg_name(..) => #msg_names.to_string(),)*
                    }
                }
                /*
                pub fn serialize(&self) -> Vec<u8> {
                    use self::#enumname::*;
                    match self {
                        #(&#enums_serialize(ref body) => body.serialize(),)*
                    }
                }
                */
            }

            #comment
            #(#msgs)*
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct PprzMessage {
    pub id: u8,
    pub name: String,
    pub link: Option<MsgLinkType>,
    pub description: Option<String>,
    pub fields: Vec<PprzField>,
}

impl PprzMessage {
    /// Return Token of "MESSAGE_NAME_DATA
    /// for pprzlink struct data
    fn emit_struct_name(&self) -> Tokens {
        let name = Ident::from(format!("{}_DATA", self.name));
        quote!(#name)
    }

    fn emit_enum_name(&self) -> Tokens {
        let name = Ident::from(format!("{}", self.name));
        quote!(#name)
    }

    fn emit_msg_name(&self) -> Tokens {
        let name = Ident::from(format!("\"{}\"", self.name));
        quote!(#name)
    }

    fn emit_description(&self) -> Ident {
        match &self.description {
            Some(val) => {
                let val = val.split_whitespace().collect::<Vec<&str>>().as_slice().join(" ");
                return Ident::from(format!("\n/* {} */\n",val));
            },
            None => Ident::from(String::new()),
        }
    }

    fn emit_name_types(&self) -> Vec<Tokens> {
        self.fields
            .iter()
            .map(|field| {
                let nametype = field.emit_name_type();
                let description = field.emit_description();
                quote!{
                    #description
                    #nametype
                }
            })
            .collect::<Vec<Tokens>>()
    }

    fn emit_rust(&self) -> Tokens {
        let msg_name = self.emit_struct_name();
        let name_types = self.emit_name_types();
        let description = self.emit_description();

        quote!{
            #description
            #[derive(Clone, PartialEq, Default, Debug)]
            #[derive(Serialize, Deserialize)]
            pub struct #msg_name {
                #(#name_types)*
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum MsgLinkType {
    Broadcasted,
    Forwarded
}

impl MsgLinkType {
    pub fn idendify_link_type(s: &str) -> MsgLinkType {
        use self::MsgLinkType::*;
        match s {
            "broadcasted" => Broadcasted,
            "forwarded" => Forwarded,
            _ => panic!("Unrecognized link type"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct PprzField {
    pub fieldtype: PprzType,
    pub name: String,
    pub description: Option<String>,
    pub format: Option<String>,
    pub unit: Option<String>,
    pub alt_unit: Option<String>,
    pub alt_unit_coef: Option<f32>,
    pub values: Option<String>,
}

impl PprzField {
    fn emit_name(&self) -> Tokens {
        let name = Ident::from(self.name.clone());
        quote!(#name)
    }

    fn emit_type(&self) -> Tokens {
        let fieldtype = Ident::from(self.fieldtype.rust_type());
        quote!(#fieldtype)
    }

    fn emit_description(&self) -> Tokens {
        let desc = match &self.description {
            Some(val) => Ident::from(format!("\n/* {}*/\n",val)),
            None => Ident::from(String::new()),
        };
        quote!(#desc)
    }

    fn emit_name_type(&self) -> Tokens {
        let name = self.emit_name();
        let fieldtype = self.emit_type();
        quote!(pub #name: #fieldtype,)
    }
}

/// Generate rust representation of pprzlink message set with appropriate conversion methods
//pub fn generate<R: Read, W: Write>(input: &mut R, output_rust: &mut W) {
pub fn generate<R: Read>(input: &mut R, output_rust_path: &Path) {
    let profile = parse_profile(input);

    // rust file
    //let rust_tokens = profile.emit_rust();
    //println!("{}", rust_tokens);
    //writeln!(output_rust, "{}", rust_tokens).unwrap();
    /*
    let rust_src = rust_tokens.into_string();
    let mut cfg = rustfmt::config::Config::default();
    cfg.set().write_mode(rustfmt::config::WriteMode::Display);
    rustfmt::format_input(rustfmt::Input::Text(rust_src), &cfg, Some(output_rust)).unwrap();
    */    
    
    for (class_name, rust_tokens) in profile.emit_classes().iter().zip(profile.emit_msgs().iter()) {
        let rust_src = rust_tokens.clone().into_string();
        let mut cfg = rustfmt::config::Config::default();
        cfg.set().write_mode(rustfmt::config::WriteMode::Display);

        let dest_path_rust = output_rust_path.join(class_name.to_string() + ".rs");
        let mut output_rust = File::create(&dest_path_rust).unwrap();
        rustfmt::format_input(rustfmt::Input::Text(rust_src), &cfg, Some(&mut output_rust)).unwrap();
    }    
}

/// XML parsing related type
#[derive(Debug, PartialEq, Clone)]
pub enum PprzType {
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Int8,
    Int16,
    Int32,
    Int64,
    Char,
    Float,
    Double,
    PprzString,
    Array(Box<PprzType>, usize),
}

impl Default for PprzType {
    fn default() -> PprzType {
        PprzType::UInt8
    }
}

impl PprzType {
    /// Return rust equivalent of a given Mavtype
    /// Used for generating struct fields.
    /// Note, the smallest type is u32 to make it compatible
    /// with protobuf protocol
    pub fn rust_type(&self) -> String {
        use self::PprzType::*;
        match self.clone() {
            UInt8 | Char => "u8".into(),
            UInt16 => "u16".into(),
            UInt32 => "u32".into(),
            Int8 => "i8".into(),
            Int16 => "i16".into(),
            Int32 => "i32".into(),
            Float => "f32".into(),
            UInt64 => "u64".into(),
            Int64 => "i64".into(),
            Double => "f64".into(),
            PprzString => "String".into(),
            Array(t, size) => match size {
                0 => format!("Vec<{}> /* unspecified */", t.rust_type()),
                _ => format!("Vec<{}> /* {} */", t.rust_type(), size),
            }
                
        }
    }

}

/// If the array is unbounded, size is set to zero
/// and any subsequent handling should reflect that
fn parse_type(s: &str) -> Option<PprzType> {
    use self::PprzType::*;
    match s {
        "uint8" => Some(UInt8),
        "uint16" => Some(UInt16),
        "uint32" => Some(UInt32),
        "uint64" => Some(UInt64),
        "int8" => Some(Int8),
        "int16" => Some(Int16),
        "int32" => Some(Int32),
        "int64" => Some(Int64),
        "char" => Some(Char),
        "float" => Some(Float),
        "double" => Some(Double),
        "string" => Some(PprzString),
        _ => {
            if s.ends_with("]") {
                let start = s.find("[").unwrap();
                let size = match s[start + 1..(s.len() - 1)].parse::<usize>() {
                    Ok(val) => val,
                    Err(_) => 0,
                };
                let mtype = parse_type(&s[0..start]).unwrap();
                Some(Array(Box::new(mtype), size))
            } else {
                panic!("UNHANDLED {:?}", s);
            }
        }
    }
}

/// Parse XML and return a set of parsed messages
pub fn parse_profile(file: &mut Read) -> PprzProfile {
    let mut stack: Vec<PprzXmlElement> = vec![];

    let mut profile = PprzProfile {
        msg_classes: HashMap::new(),
    };

    let mut field: PprzField = Default::default();
    let mut message: PprzMessage = Default::default();
    let mut msg_class: PprzMsgClass = Default::default();

    let parser = EventReader::new(file);
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement {
                name,
                attributes: attrs,
                ..
            }) => {
                let id = match identify_element(&name.to_string()) {
                    None => {
                        panic!("unexpected element {:?}", name);
                    }
                    Some(kind) => kind,
                };

                if !is_valid_parent(
                    match stack.last().clone() {
                        Some(arg) => Some(arg.clone()),
                        None => None,
                    },
                    id.clone(),
                ) {
                    panic!("not valid parent {:?} of {:?}", stack.last(), id);
                }

                match id {
                    PprzXmlElement::Message => {
                        message = Default::default();
                    }
                    PprzXmlElement::Field => {
                        field = Default::default();
                    }
                    PprzXmlElement::MsgClass => {
                        msg_class = Default::default();
                    }
                    _ => (),
                }
                stack.push(id);

                for attr in attrs {
                    match stack.last() {
                        Some(&PprzXmlElement::MsgClass) => {
                            match attr.name.local_name.clone().as_ref() {
                                "name" => {
                                    msg_class.name = attr.value.clone();
                                }
                                "id" => {
                                    msg_class.id = attr.value.parse::<u8>().unwrap();
                                }
                                _ => (),
                            }
                        }
                        Some(&PprzXmlElement::Message) => {
                            match attr.name.local_name.clone().as_ref() {
                                "name" => {
                                    /*
                                    // For SnakeCaseMessageNames
                                    println!("{}", attr.value);
                                    message.name = attr
                                        .value
                                        .clone()
                                        .split("_")
                                        .map(|x| x.to_lowercase())
                                        .map(|x| {
                                            let mut v: Vec<char> = x.chars().collect();
                                            if !v.is_empty() {
                                                v[0] = v[0].to_uppercase().nth(0).unwrap();
                                            }
                                            v.into_iter().collect()
                                        })
                                        .collect::<Vec<String>>()
                                        .join("");
                                    */
                                    message.name = attr.value.clone();
                                }
                                "id" => {
                                    message.id = attr.value.parse::<u8>().unwrap();
                                }
                                "link" => {
                                    message.link = Some(MsgLinkType::idendify_link_type(&attr.value));
                                }
                                _ => (),
                            }
                        }
                        Some(&PprzXmlElement::Field) => {
                            match attr.name.local_name.clone().as_ref() {
                                "name" => {
                                    field.name = attr.value.clone();
                                    if field.name == "type" {
                                        field.name = "fieldtype".to_string();
                                    }
                                }
                                "type" => {
                                    field.fieldtype = parse_type(&attr.value).unwrap();
                                }
                                "format" => {
                                    field.format = Some(attr.value);
                                }
                                "unit" => {
                                    field.unit = Some(attr.value);
                                }
                                "values" => {
                                    field.values = Some(attr.value);
                                }
                                "alt_unit" => {
                                    field.alt_unit = Some(attr.value);
                                }
                                "alt_unit_coef" => {
                                    field.alt_unit_coef = Some(attr.value.parse::<f32>().unwrap());
                                }
                                _ => (),
                            }
                        }
                        _ => (),
                    }
                }
            }
            // Handle any PCDATA between tags
            Ok(XmlEvent::Characters(s)) => {
                use self::PprzXmlElement::*;
                match (stack.last(), stack.get(stack.len() - 2)) {
                    (Some(&Description), Some(&Message)) => {
                        message.description = Some(s);
                    }
                    (Some(&Field), Some(&Message)) => {
                        field.description = Some(s);
                    }
                    data => {
                        panic!("unexpected text data {:?} reading {:?}", data, s);
                    }
                }
            }
            Ok(XmlEvent::EndElement { .. }) => {
                match stack.last() {
                    Some(&PprzXmlElement::Field) => message.fields.push(field.clone()),
                    Some(&PprzXmlElement::Message) => {
                        let msg = message.clone();
                        msg_class.messages.push(msg);
                    }
                    Some(&PprzXmlElement::MsgClass) => {
                        let name = msg_class.name.clone();
                        let id = msg_class.id.clone();
                        profile.msg_classes.insert((id, name), msg_class.clone());
                    }
                    _ => (),
                }
                stack.pop();
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
    profile
}
