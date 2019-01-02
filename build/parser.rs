use changecase::ChangeCase;
use std::collections::HashMap;
use std::default::Default;
//use std::io::Write;
use std::io::Read;
use std::path::Path;
use std::fs::File;
use std::string::ToString;

use xml::reader::{EventReader, XmlEvent};

use quote::{Ident, Tokens};

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
    fn emit_msgs(&self) -> Vec<Tokens> {
        self.msg_classes.values()
            .map(|d| d.emit_rust())
            .collect::<Vec<Tokens>>()
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
    fn emit_serde_derive() -> Ident {
        #[cfg(feature = "serde-derive")]
        return Ident::from("#[derive(Serialize, Deserialize)]");
        #[cfg(not(feature = "serde-derive"))]
        return Ident::from("");
    }

    fn emit_includes() -> Tokens {
        let common = quote!{
            use bytes::{Buf, BufMut, Bytes, IntoBuf};
        };
        #[cfg(not(feature = "std"))]
        return quote!{
            #common
            use alloc::fmt;
            use alloc::string::String;
            use alloc::str::Chars;
            use alloc::vec;
            use alloc::vec::Vec;
            use alloc::prelude::ToString;
        };
        #[cfg(feature = "std")]
        return quote!{
            #common
            use std::fmt;
            use std::str::Chars;
        };
    }

    fn emit_comments(&self) -> Ident {
        Ident::from(format!(
            "\n/* This file was automatically generated, do not edit */\n"
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
        let structs_from_str = structs.clone();

        let enums = self.emit_enum_names();
        let enums_parse = enums.clone();
        let enums_serialize = enums.clone();
        let enums_display = enums.clone();
        let enums_from_str = enums.clone();

        let msg_ids = self.emit_msg_ids();
        let msg_ids_parse = msg_ids.clone();
        let msg_ids_ser = msg_ids.clone();
        
        let msg_names = self.emit_msg_names();
        let msg_names_from_str = msg_names.clone();

        let derive_serde = PprzMsgClass::emit_serde_derive();
        let includes = PprzMsgClass::emit_includes();

        quote!{
            #comment
            #[allow(non_camel_case_types)]
            #[allow(non_snake_case)]
            pub mod #modname {
                #includes

            #[derive(Clone, PartialEq, Debug)]
            #derive_serde
            pub enum #enumname {
                #(#enums(#structs)),*
            }

            pub fn create_ivy_message(msg: &#enumname, sender: &str) -> String {
                sender.to_string() + " " + &msg.to_string()
            }

            // TODO: should return the sender
            pub fn parse_ivy_msg_from_sender(input: &str, sender: Option<&str>) -> Option<#enumname> {
                let mut input = input.chars();
                let parsed_sender: String = input.by_ref().take_while(|x| *x!=' ').collect();
                if let Some(expected_sender) = sender {
                    if parsed_sender != expected_sender {
                        // Senders don't match
                        return None;
                    }
                }
                #enumname::from_chars(&mut input)
            }

            impl #enumname {
                pub fn deser(payload: &[u8]) -> Option<#enumname> {
                    use self::#enumname::*;
                    let id = payload[0];
                    match id {
                        #(#msg_ids_parse => Some(#enums_parse(#structs_parse::deser(&payload[1..]).unwrap())),)*
                        _ => None,
                    }
                }

                pub fn from_str(s: &str) -> Option<#enumname> {
                    let mut input = s.chars();
                    #enumname::from_chars(&mut input)
                }
                
                fn from_chars(mut input: &mut Chars) -> Option<#enumname> {
                    let name: String = input.by_ref().take_while(|x| *x!=' ').collect();
                    use self::#enumname::*;
                    match name.as_ref() {
                        #(#msg_names_from_str => Some( #enums_from_str( #structs_from_str :: from_chars(&mut input).unwrap() )), )*
                        _ => None
                    }
                }
                
                pub fn ser(&self) -> Vec<u8> {
                    let mut v = Vec::new();
                    use self::#enumname::*;
                    match self {
                        #(&#enums_serialize(ref body) => {
                                v.push(#msg_ids_ser);
                                v.append(&mut body.ser());
                            },)*
                    }
                    v
                }
            }

            impl fmt::Display for #enumname {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    use self::#enumname::*;
                    match self {
                        #(#enums_display(m) => m.fmt(f),)*
                    }
                }
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

    
    fn emit_from_str(&self) -> Tokens {
        let from_str = self.fields.iter()
            .map(|f| {
                let name = f.emit_name();
                use self::PprzType::*;
                match &f.fieldtype {
                    Array(t) => {
                        let t = Ident::from(t.rust_type());
                        quote!{
                            loop {
                                let val: String = _tmp.by_ref().take_while(|x| *x!=',').collect();
                                if val.is_empty() {
                                    break;
                                }
                                _struct.#name.push(val.parse::<#t>().unwrap())
                            }
                        }
                    }
                    Slice(t,l) => {
                        let t = Ident::from(t.rust_type());
                        let l = Ident::from(l.to_string());
                        quote!{
                            for idx in 0..#l {
                                let val: String = _tmp.by_ref().take_while(|x| *x!=',').collect();
                                _struct.#name[idx] = val.parse::<#t>().unwrap();
                            }
                        }
                    },
                    _ => {
                        let t = Ident::from(f.fieldtype.clone().rust_type());
                        quote!{
                            let val: String = _tmp.by_ref().take_while(|x| *x!=' ').collect();
                            _struct.#name = val.parse::<#t>().unwrap();
                        }
                    }
                }
            }).collect::<Vec<Tokens>>();
        quote!{
            let mut _struct = Self::default();
            #(#from_str)*
            Some(_struct)
        }
    }

    fn emit_serialize_vars(&self) -> Tokens {
        let ser_vars = self.fields.iter()
            .map(|f| {
                let name = Ident::from("self.".to_string() + &f.name.clone());
                let buf = Ident::from("_tmp");
                f.fieldtype.rust_writer(name, buf)
            }).collect::<Vec<Tokens>>();
            quote!{
                let mut _tmp = vec![];
                #(#ser_vars)*
                _tmp
            }
    }

    fn emit_deserialize_vars(&self) -> Tokens {
        let deser_vars = self.fields.iter()
            .map(|f| {
                let name = Ident::from("_struct.".to_string() + &f.name.clone());
                let buf = Ident::from("buf");
                f.fieldtype.rust_reader(name, buf)
            }).collect::<Vec<Tokens>>();
            if deser_vars.is_empty() {
                // struct has no fields
                quote!{
                    Some(Self::default())
                }
            } else {
                quote!{
                    let mut buf = Bytes::from(_input).into_buf();
                    let mut _struct = Self::default();
                    #(#deser_vars)*
                    Some(_struct)
                }
            }
    }

    fn emit_display(&self) -> Tokens {
        let to_string = self.fields.iter()
            .map(|f| {
                let name = f.emit_name();
                use self::PprzType::*;
                match &f.fieldtype {
                    Array(_t) => {
                        quote!{
                            for val in &self.#name {
                                tmp += &val.to_string();
                                tmp+=",";
                            }
                        }
                    },
                    Slice(_t,_l) => {
                        quote!{
                            for val in self.#name.iter() {
                                tmp += &val.to_string();
                                tmp+=",";
                            }
                        }
                    }
                    _ => quote!{
                        tmp += &self.#name.to_string();
                        tmp+=" ";
                    }
                }
            }).collect::<Vec<Tokens>>();
        let name = self.emit_msg_name();
        quote!{
            let mut tmp = String::new();
            tmp += &#name;
            tmp += " ";
            #(#to_string)*
            write!(f, "{}", tmp)
        }
    }

    fn emit_rust(&self) -> Tokens {
        let msg_name = self.emit_struct_name();
        let name_types = self.emit_name_types();
        let description = self.emit_description();
        let display = self.emit_display();
        let from_str = self.emit_from_str();
        let derive_serde = PprzMsgClass::emit_serde_derive();
        let serialize_vars = self.emit_serialize_vars();
        let deser_vars = self.emit_deserialize_vars();

        quote!{
            #description
            #[derive(Clone, PartialEq, Default, Debug)]
            #derive_serde
            pub struct #msg_name {
                #(#name_types)*
            }

            impl fmt::Display for #msg_name {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    #display
                }
            }

            impl #msg_name {
                pub fn from_chars(_tmp: &mut Chars) -> Option<Self> {
                    #from_str
                }

                pub fn ser(&self) -> Vec<u8> {
                    #serialize_vars
                }

                pub fn deser(_input: &[u8]) -> Option<Self> {
                    #deser_vars
                }
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
        let mut desc = vec![];
        if let Some(val) = self.description.clone() {
            desc.push(format!("\n/// {}.",val));
        }
        if let Some(val) = self.format.clone() {
            desc.push(format!("\n/// format: {}.",val));
        }
        if let Some(val) = self.unit.clone() {
            desc.push(format!("\n/// unit: {}.",val));
        }
        if let Some(val) = self.alt_unit.clone() {
            desc.push(format!("\n/// alt_unit: {}.",val));
        }
        if let Some(val) = self.alt_unit_coef.clone() {
            desc.push(format!("\n/// alt_unit_coef: {}.",val));
        }
        if let Some(val) = self.values.clone() {
            desc.push(format!("\n/// values: {:?}.",val));
        }
        desc.push("\n".to_string());
        let desc: String = desc.iter().map(|s| s.to_string()).collect();
        let desc = Ident::from(desc);
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
    let mut cfg = rustfmt::config::Config::default();
    cfg.set().write_mode(rustfmt::config::WriteMode::Display);  
    
    for (class_name, rust_tokens) in profile.emit_classes().iter().zip(profile.emit_msgs().iter()) {
        let rust_src = rust_tokens.clone().into_string();
        
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
    Slice(Box<PprzType>, usize),
    Array(Box<PprzType>),
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
            UInt8 => "u8".into(),
            Char => "char".into(),
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
            Slice(t, size) => format!("[{};{}] /* fixed length array */", t.rust_type(), size),
            Array(t) => format!("Vec<{}> /* arbitrary length array */", t.rust_type()),
        }
    }

    pub fn rust_reader(&self, val: Ident, buf: Ident) -> Tokens {
        use self::PprzType::*;
        match self.clone() {
            Char => quote!{#val = #buf.get_u8() as char;},
            UInt8 => quote!{#val = #buf.get_u8();},
            UInt16 => quote!{#val = #buf.get_u16_le();},
            UInt32 => quote!{#val = #buf.get_u32_le();},
            UInt64 => quote!{#val = #buf.get_u64_le();},
            Int8 => quote!{#val = #buf.get_i8();},
            Int16 => quote!{#val = #buf.get_i16_le();},
            Int32 => quote!{#val = #buf.get_i32_le();},
            Int64 => quote!{#val = #buf.get_i64_le();},
            Float => quote!{#val = #buf.get_f32_le();},
            Double => quote!{#val = #buf.get_f64_le();},
            PprzString => quote!{
                let s_len = #buf.get_u8() as usize;
                let mut s_vec = Vec::with_capacity(s_len);
                for _ in 0..s_len {
                    s_vec.push(#buf.get_u8());
                }
                #val = String::from_utf8(s_vec).unwrap();
            },
            Array(t) => {
                    let r = t.rust_reader(Ident::from("let val"), buf.clone());
                    quote!{
                        let s_len = #buf.get_u8() as usize;
                        for _ in 0..s_len {
                            #r
                            #val.push(val);
                    }
                }
            },
            Slice(t, _size) => {
                let r = t.rust_reader(Ident::from("let val"), buf.clone());
                quote!{
                    for idx in 0..#val.len() {
                        #r
                        #val[idx] = val;
                    }
                }
            }
        }
    }

    pub fn rust_writer(&self, val: Ident, buf: Ident) -> Tokens {
        use self::PprzType::*;
        match self.clone() {
            UInt8 => quote!{#buf.put_u8(#val);},
            Char => quote!{#buf.put_u8(#val as u8);},
            UInt16 => quote!{#buf.put_u16_le(#val);},
            UInt32 => quote!{#buf.put_u32_le(#val);},
            Int8 => quote!{#buf.put_i8(#val);},
            Int16 => quote!{#buf.put_i16_le(#val);},
            Int32 => quote!{#buf.put_i32_le(#val);},
            Float => quote!{#buf.put_f32_le(#val);},
            UInt64 => quote!{#buf.put_u64_le(#val);},
            Int64 => quote!{#buf.put_i64_le(#val);},
            Double => quote!{#buf.put_f64_le(#val);},
            PprzString => quote!{
                #buf.put_u8(#val.as_bytes().len() as u8);
                #buf.put_slice(#val.as_bytes());
                },
            Array(t) => {
                let w = t.rust_writer(Ident::from("*val"), buf.clone());
                quote!{
                    #buf.put_u8(#val.len() as u8);
                    for val in &#val {
                        #w
                    }
                }
            },
            Slice(t, _size) => {
                let w = t.rust_writer(Ident::from("*val"), buf.clone());
                quote!{
                    for val in #val.iter() {
                        #w
                    }
                }
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
        "char[]" => Some(PprzString),
        _ => {
            if s.ends_with("]") {
                let start = s.find("[").unwrap();
                let mtype = parse_type(&s[0..start]).unwrap();
                match s[start + 1..(s.len() - 1)].parse::<usize>() {
                    Ok(size) => Some(Slice(Box::new(mtype), size)),
                    Err(_) => Some(Array(Box::new(mtype))),
                }
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
