// =========================
// Ppprzlink Messages
// =========================
extern crate xml;
use std::fs::File;
use std::io::BufReader;
use std::fmt;
use self::xml::reader::{EventReader, XmlEvent};
use self::xml::attribute::OwnedAttribute;

/// define constants
const V1_SENDER_ID: usize  = 0;
const V1_MSG_ID: usize  = 1;

const V2_SENDER_ID: usize  = 0;
const V2_DESTINATION: usize  = 1;
const V2_CLASS_COMPONENT: usize  = 2;
const V2_MSG_ID: usize  = 3;


/// two versions of pprzlink protocol
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum PprzProtocolVersion {
    ProtocolV1,
    ProtocolV2,
}

impl fmt::Display for PprzProtocolVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            PprzProtocolVersion::ProtocolV1 => String::from("v1.0"),
            PprzProtocolVersion::ProtocolV2 => String::from("v2.0"),
        };
        write!(f, "{}", s)
    }
}



/// only one version of the messages for now
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum PprzMessageVersion {
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
	Unknown = 0,
    Telemetry = 1,
    Datalink = 2,
    Ground = 3,
    Alert = 4,
    Intermcu = 5,
}

impl fmt::Display for PprzMsgClassID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
        	PprzMsgClassID::Unknown => String::from("Unknown"),
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
pub enum PprzMsgBaseType {
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
    Char(char),
    CharArr(Vec<char>),
    String(String),
}

impl fmt::Display for PprzMsgBaseType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        match *self {
            PprzMsgBaseType::Float(v) => s = format!("{}", v),
            PprzMsgBaseType::Double(v) => s = format!("{}", v),
            PprzMsgBaseType::Uint8(v) => s = format!("{}", v),
            PprzMsgBaseType::Uint16(v) => s = format!("{}", v),
            PprzMsgBaseType::Uint32(v) => s = format!("{}", v),
            PprzMsgBaseType::Int8(v) => s = format!("{}", v),
            PprzMsgBaseType::Int16(v) => s = format!("{}", v),
            PprzMsgBaseType::Int32(v) => s = format!("{}", v),
            PprzMsgBaseType::Char(v) => s = format!("{}", v),
            PprzMsgBaseType::String(ref v) => s = format!("{}", v),
            PprzMsgBaseType::FloatArr(ref v) => {
                for val in v {
                    s = s + &format!("{},", val);
                }
            }
            PprzMsgBaseType::DoubleArr(ref v) => {
                for val in v {
                    s = s + &format!("{},", val);
                }
            }
            PprzMsgBaseType::Uint8Arr(ref v) => {
                for val in v {
                    s = s + &format!("{},", val);
                }
            }
            PprzMsgBaseType::Uint16Arr(ref v) => {
                for val in v {
                    s = s + &format!("{},", val);
                }
            }
            PprzMsgBaseType::Uint32Arr(ref v) => {
                for val in v {
                    s = s + &format!("{},", val);
                }
            }
            PprzMsgBaseType::Int8Arr(ref v) => {
                for val in v {
                    s = s + &format!("{},", val);
                }
            }
            PprzMsgBaseType::Int16Arr(ref v) => {
                for val in v {
                    s = s + &format!("{},", val);
                }
            }
            PprzMsgBaseType::Int32Arr(ref v) => {
                for val in v {
                    s = s + &format!("{},", val);
                }
            }
            PprzMsgBaseType::CharArr(ref v) => {
                for val in v {
                    s = s + &format!("{}", val);
                }
            }
        }
        write!(f, "{}", s)
    }
}


/// each field has a name and a type
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PprzField {
    pub name: String,
    pub value: PprzMsgBaseType,
}

impl fmt::Display for PprzField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// Pprzlink message
/// see https://wiki.paparazziuav.org/wiki/Messages_Format
/// and https://github.com/paparazzi/pprzlink
///
/// The message doesn't contain any headers/sync or checksum,
/// it is purely representing the payload of the pprzlink message
///
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PprzMessage {
    /// specifies message protocol
    pub protocol: PprzProtocolVersion,
    /// the `SENDER_ID` of the message
    pub source: u8,
    /// the `DESTINATION` of the message (pprzlink v.2.0 only)
    pub destination: u8,
    /// the message class (`CLASS` ID), can be unknown (pprzlink v.2.0 only)
    pub class: PprzMsgClassID,
    /// the `COMPONENT` ID (pprzlink v.2.0 only)
    pub component: u8, 
    /// currently only v.1.0 is supported
    pub version: PprzMessageVersion,
    /// the `MSG_ID` in given message class
    pub id: u8,
    /// the vector of message fields
    pub fields: Vec<PprzField>,
    /// the message name, e.g. `WP_MOVED`
    pub name: String,
}

extern crate byteorder;
use std::io::Cursor;
use self::byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::mem;

use std::io::Write;



impl PprzMessage {
    /// Find whether one of the message fields contains `query`
    pub fn contains(&self, query: &str) -> bool {
        for field in &self.fields {
            if field.name == query {
                return true;
            }
        }
        return false;
    }

	/// return byte with message ID
	pub fn get_msg_id_from_buf(buf: &[u8], version: PprzProtocolVersion) -> u8 {
		match version {
			PprzProtocolVersion::ProtocolV1 => {
				buf[V1_MSG_ID]
			}
			PprzProtocolVersion::ProtocolV2 => {
				buf[V2_MSG_ID]
			}
		}
	}

    /// Create a new empty message, all fields set to zero,
    /// all vectors are empty
    pub fn new() -> PprzMessage {
        PprzMessage {
            protocol: PprzProtocolVersion::ProtocolV2,
            source: 0, // SENDER_ID
            destination: 0, // can be BROADCAST
            component: 0,
            version: PprzMessageVersion::MessagesV1,
            id: 0, // MSG_ID
            fields: vec![],
            name: String::new(),
            class: PprzMsgClassID::Unknown,
        }
    }

	/// Parsing a string to a PPRZ message
	/// the format is:
	/// "SENDER"
	/// "MSG_NAME"
	/// 0-N "FIELDS"
    pub fn update_from_string(&mut self, payload: &Vec<&str>) {
    	if payload.len() < 2 {
			// message contains no payload (only SENDER and MSG_NAME)
		    return;
    	}

		// attempt to update the sender field (first in the list)
		self.source = payload[0].parse::<u8>().unwrap_or(0);

    	let mut idx = 2;

        for field in &mut self.fields {
            match field.value {
                PprzMsgBaseType::Uint8(_) => {
                    field.value = PprzMsgBaseType::Uint8(payload[idx].parse::<u8>().unwrap());
                    idx += 1;
                }
                PprzMsgBaseType::Uint16(_) => {
                    field.value = PprzMsgBaseType::Uint16(payload[idx].parse::<u16>().unwrap());
                    idx += 1;
                }
                PprzMsgBaseType::Uint32(_) => {
                    field.value = PprzMsgBaseType::Uint32(payload[idx].parse::<u32>().unwrap());
                    idx += 1;
                }
                PprzMsgBaseType::Int8(_) => {
                    field.value = PprzMsgBaseType::Int8(payload[idx].parse::<i8>().unwrap());
                    idx += 1;
                }
                PprzMsgBaseType::Int16(_) => {
                    field.value = PprzMsgBaseType::Int16(payload[idx].parse::<i16>().unwrap());
                    idx += 1;
                }
                PprzMsgBaseType::Int32(_) => {
                    field.value = PprzMsgBaseType::Int32(payload[idx].parse::<i32>().unwrap());
                    idx += 1;
                }
                PprzMsgBaseType::Float(_) => {
                    field.value = PprzMsgBaseType::Float(payload[idx].parse::<f32>().unwrap());
                    idx += 1;
                }
                PprzMsgBaseType::Double(_) => {
                    field.value = PprzMsgBaseType::Double(payload[idx].parse::<f64>().unwrap());
                    idx += 1;
                }
                PprzMsgBaseType::Char(_) => {
                    field.value = PprzMsgBaseType::Char(payload[idx].parse::<char>().unwrap());
                    idx += 1;
                }
                PprzMsgBaseType::CharArr(_) => {
                    // Ivy message doesnt have the `len` attribute
                    // it looks like '1 COMMANDS 0,0,0,0,0'
                    // We have to assume that the array is the last field, thus the remaining of
                    // the payload is for the array
                    // We also have to assume that this is the last field in the `fields`
                    //
                    // For character array, when parsing from Ivy bus message string
                    // we want to remove all {'\','"'} characters that cause problem
                    // for OCaml ivy parser (for example '\"v5.13_devel-85-gddc8510-dirty\"'
                    // have to become 'v5.13_devel-85-gddc8510-dirty' 
                    let mut data = vec![];
                    let mychararray = payload[idx].as_bytes();
                    for k in 0..mychararray.len() {
                    	let c = mychararray[k] as char;
                    	if c == '\\' || c == '\"' {
                    		// skip this character
                    		continue;
                    	}
                    	data.push(c);
                    }
                    field.value = PprzMsgBaseType::CharArr(data);
                }
                PprzMsgBaseType::Uint8Arr(_) => {
                    // Ivy message doesnt have the `len` attribute
                    // it looks like '1 COMMANDS 0,0,0,0,0'
                    // We have to assume that the array is the last field, thus the remaining of
                    // the payload is for the array
                    // We also have to assume that this is the last field in the `fields`
                    //
                    // We have to take care of empty elements as well, so for example
                    // ["2", "MISSION_STATUS", "-1", "0", ""] will produce a data vec = [0]
                    let mut data = vec![];
                    for k in idx..payload.len() {
                    	if payload[k].is_empty() {
                    		continue;
                    	}
                        data.push(payload[k].parse::<u8>().unwrap());
                    }
                    field.value = PprzMsgBaseType::Uint8Arr(data);
                }
                PprzMsgBaseType::Uint16Arr(_) => {
                    // Ivy message doesnt have the `len` attribute
                    // it looks like '1 COMMANDS 0,0,0,0,0'
                    // We have to assume that the array is the last field, thus the remaining of
                    // the payload is for the array
                    // We also have to assume that this is the last field in the `fields`
                    //
                    // We have to take care of empty elements as well, so for example
                    // ["2", "MISSION_STATUS", "-1", "0", ""] will produce a data vec = [0]
                    let mut data = vec![];
                    for k in idx..payload.len() {
                    	if payload[k].is_empty() {
                    		continue;
                    	}
                        data.push(payload[k].parse::<u16>().unwrap());
                    }
                    field.value = PprzMsgBaseType::Uint16Arr(data);
                }
                PprzMsgBaseType::Uint32Arr(_) => {
                    // Ivy message doesnt have the `len` attribute
                    // it looks like '1 COMMANDS 0,0,0,0,0'
                    // We have to assume that the array is the last field, thus the remaining of
                    // the payload is for the array
                    // We also have to assume that this is the last field in the `fields`
                    let mut data = vec![];
                    for k in idx..payload.len() {
                        data.push(payload[k].parse::<u32>().unwrap());
                    }
                    field.value = PprzMsgBaseType::Uint32Arr(data);
                }
                PprzMsgBaseType::Int8Arr(_) => {
                    // Ivy message doesnt have the `len` attribute
                    // it looks like '1 COMMANDS 0,0,0,0,0'
                    // We have to assume that the array is the last field, thus the remaining of
                    // the payload is for the array
                    // We also have to assume that this is the last field in the `fields`
                    //
                    // We have to take care of empty elements as well, so for example
                    // ["2", "MISSION_STATUS", "-1", "0", ""] will produce a data vec = [0]
                    let mut data = vec![];
                    for k in idx..payload.len() {
                    	if payload[k].is_empty() {
                    		continue;
                    	}
                        data.push(payload[k].parse::<i8>().unwrap());
                    }
                    field.value = PprzMsgBaseType::Int8Arr(data);
                }
                PprzMsgBaseType::Int16Arr(_) => {
                    // Ivy message doesnt have the `len` attribute
                    // it looks like '1 COMMANDS 0,0,0,0,0'
                    // We have to assume that the array is the last field, thus the remaining of
                    // the payload is for the array
                    // We also have to assume that this is the last field in the `fields`
                    //
                    // We have to take care of empty elements as well, so for example
                    // ["2", "MISSION_STATUS", "-1", "0", ""] will produce a data vec = [0]
                    let mut data = vec![];
                    for k in idx..payload.len() {
                    	if payload[k].is_empty() {
                    		continue;
                    	}
                        data.push(payload[k].parse::<i16>().unwrap());
                    }
                    field.value = PprzMsgBaseType::Int16Arr(data);
                }
                PprzMsgBaseType::Int32Arr(_) => {
                    // Ivy message doesnt have the `len` attribute
                    // it looks like '1 COMMANDS 0,0,0,0,0'
                    // We have to assume that the array is the last field, thus the remaining of
                    // the payload is for the array
                    // We also have to assume that this is the last field in the `fields`
                    //
                    // We have to take care of empty elements as well, so for example
                    // ["2", "MISSION_STATUS", "-1", "0", ""] will produce a data vec = [0]
                    let mut data = vec![];
                    for k in idx..payload.len() {
                    	if payload[k].is_empty() {
                    		continue;
                    	}
                        data.push(payload[k].parse::<i32>().unwrap());
                    }
                    field.value = PprzMsgBaseType::Int32Arr(data);
                }
                PprzMsgBaseType::FloatArr(_) => {
                    // Ivy message doesnt have the `len` attribute
                    // it looks like '1 COMMANDS 0,0,0,0,0'
                    // We have to assume that the array is the last field, thus the remaining of
                    // the payload is for the array
                    // We also have to assume that this is the last field in the `fields`
                    //
                    // We have to take care of empty elements as well, so for example
                    // ["2", "MISSION_STATUS", "-1", "0", ""] will produce a data vec = [0]
                    let mut data = vec![];
                    for k in idx..payload.len() {
                    	if payload[k].is_empty() {
                    		continue;
                    	}
                        data.push(payload[k].parse::<f32>().unwrap());
                    }
                    field.value = PprzMsgBaseType::FloatArr(data);
                }
                PprzMsgBaseType::DoubleArr(_) => {
                    // Ivy message doesnt have the `len` attribute
                    // it looks like '1 COMMANDS 0,0,0,0,0'
                    // We have to assume that the array is the last field, thus the remaining of
                    // the payload is for the array
                    // We also have to assume that this is the last field in the `fields`
                    //
                    // We have to take care of empty elements as well, so for example
                    // ["2", "MISSION_STATUS", "-1", "0", ""] will produce a data vec = [0]
                    let mut data = vec![];
                    for k in idx..payload.len() {
                    	if payload[k].is_empty() {
                    		continue;
                    	}
                        data.push(payload[k].parse::<f64>().unwrap());
                    }
                    field.value = PprzMsgBaseType::DoubleArr(data);
                }
                PprzMsgBaseType::String(_) => panic!("String is a currently unsuported type"),
            }
        }
    }

    /// Fill in the message with real data from the `payload`.
    /// Payload is expected to be in the format of:
    ///
    /// Pprzlink 1.0
    /// ```ignore
    /// payload[0] SENDER_ID
    /// payload[1] MSG_ID
    /// payload[2-end] MSG_PAYLOAD
    /// ```
    ///
    /// Pprzlink 2.0
    /// ```ignore
    /// payload[0] source SENDER_ID
    /// payload[1] destination ID
    /// payload[2] class/component
    /// payload[3] MSG_ID
    /// payload[4-end] MSG_PAYLOAD
    /// ```
    /// 
    pub fn update(&mut self, payload: &[u8]) {
        self.source = payload[0];
        let mut idx;
        match self.protocol {
        	PprzProtocolVersion::ProtocolV1 => {
        		idx = 2;
        	}
        	PprzProtocolVersion::ProtocolV2 => {
        		idx = 4;
        	}
        }

        for field in &mut self.fields {
            // check out of bounds condition
            if idx > payload.len() {
                panic!("Error in update: idx= {} > len={} !", idx, payload.len());
            }

            // match field values
            match field.value {
                PprzMsgBaseType::Uint8(_) => {
                    field.value = PprzMsgBaseType::Uint8(payload[idx]);
                    idx += 1;
                }
                PprzMsgBaseType::Uint16(_) => {
                    let size = mem::size_of::<u16>();
                    let mut rdr = Cursor::new(&payload[idx..idx + size]);
                    match rdr.read_u16::<LittleEndian>() {
                        Ok(v) => field.value = PprzMsgBaseType::Uint16(v),
                        Err(e) => {
                            field.value = PprzMsgBaseType::Uint16(0);
                            println!("Error updating message field {} {}: {}",
                                     field.name,
                                     field.value,
                                     e);
                        }
                    };
                    idx += size;
                }
                PprzMsgBaseType::Uint32(_) => {
                    let size = mem::size_of::<u32>();
                    let mut rdr = Cursor::new(&payload[idx..idx + size]);
                    match rdr.read_u32::<LittleEndian>() {
                        Ok(v) => field.value = PprzMsgBaseType::Uint32(v),
                        Err(e) => {
                            field.value = PprzMsgBaseType::Uint32(0);
                            println!("Error updating message field {} {}: {}",
                                     field.name,
                                     field.value,
                                     e);
                        }
                    };
                    idx += size;
                }
                PprzMsgBaseType::Int8(_) => {
                    field.value = PprzMsgBaseType::Int8(payload[idx] as i8);
                    idx += 1;
                }
                PprzMsgBaseType::Int16(_) => {
                    let size = mem::size_of::<i16>();
                    let mut rdr = Cursor::new(&payload[idx..idx + size]);
                    match rdr.read_i16::<LittleEndian>() {
                        Ok(v) => field.value = PprzMsgBaseType::Int16(v),
                        Err(e) => {
                            field.value = PprzMsgBaseType::Int16(0);
                            println!("Error updating message field {} {}: {}",
                                     field.name,
                                     field.value,
                                     e);
                        }
                    };
                    idx += size;
                }
                PprzMsgBaseType::Int32(_) => {
                    let size = mem::size_of::<i32>();
                    let mut rdr = Cursor::new(&payload[idx..idx + size]);
                    match rdr.read_i32::<LittleEndian>() {
                        Ok(v) => field.value = PprzMsgBaseType::Int32(v),
                        Err(e) => {
                            field.value = PprzMsgBaseType::Int32(0);
                            println!("Error updating message field {} {}: {}",
                                     field.name,
                                     field.value,
                                     e);
                        }
                    };
                    idx += size;
                }
                PprzMsgBaseType::Float(_) => {
                    let size = mem::size_of::<f32>();
                    let mut rdr = Cursor::new(&payload[idx..idx + size]);
                    // TODO: stm32f4 is little endian, make as default?
                    // or use network endiannes?
                    match rdr.read_f32::<LittleEndian>() {
                        Ok(v) => field.value = PprzMsgBaseType::Float(v),
                        Err(e) => {
                            field.value = PprzMsgBaseType::Float(0.0);
                            println!("Error updating message field {} {}: {}",
                                     field.name,
                                     field.value,
                                     e);
                        }
                    };
                    idx += size;
                }
                PprzMsgBaseType::Double(_) => {
                    let size = mem::size_of::<f64>();
                    let mut rdr = Cursor::new(&payload[idx..idx + size]);
                    match rdr.read_f64::<LittleEndian>() {
                        Ok(v) => field.value = PprzMsgBaseType::Double(v),
                        Err(e) => {
                            field.value = PprzMsgBaseType::Double(0.0);
                            println!("Error updating message field {} {}: {}",
                                     field.name,
                                     field.value,
                                     e);
                        }
                    };
                    idx += size;
                }
                PprzMsgBaseType::Char(_) => {
                    field.value = PprzMsgBaseType::Char(payload[idx] as char);
                    idx += 1;
                }
                PprzMsgBaseType::CharArr(_) => {
                    let len = payload[idx] as usize;
                    idx += 1;

                    let mut data = vec![];
                    for k in idx..idx + len {
                        data.push(payload[k] as char);
                        idx += 1;
                    }
                    field.value = PprzMsgBaseType::CharArr(data);
                }
                PprzMsgBaseType::Uint8Arr(_) => {
                    let len = payload[idx] as usize;
                    idx += 1;

                    let mut data = vec![];
                    for k in idx..idx + len {
                        data.push(payload[k]);
                        idx += 1;
                    }
                    field.value = PprzMsgBaseType::Uint8Arr(data);
                }
                PprzMsgBaseType::Uint16Arr(_) => {
                    let len = payload[idx] as usize; // length of the array
                    let size = mem::size_of::<u16>(); // size of one element
                    idx += 1; // increment index

                    let mut data = vec![]; // init empty vector
                    let end = idx + len * size;

                    let mut rdr = Cursor::new(&payload[idx..end]);
                    for _ in 0..len {
                        match rdr.read_u16::<LittleEndian>() {
                            Ok(v) => data.push(v),
                            Err(e) => {
                                println!("Error updating message field {} {}: {}",
                                         field.name,
                                         field.value,
                                         e);
                            }
                        };
                        idx += size;
                    }
                    field.value = PprzMsgBaseType::Uint16Arr(data);
                }
                PprzMsgBaseType::Uint32Arr(_) => {
                    let len = payload[idx] as usize; // length of the array
                    let size = mem::size_of::<u32>(); // size of one element
                    idx += 1; // increment index

                    let mut data = vec![]; // init empty vector
                    let end = idx + len * size;

                    let mut rdr = Cursor::new(&payload[idx..end]);
                    for _ in 0..len {
                        match rdr.read_u32::<LittleEndian>() {
                            Ok(v) => data.push(v),
                            Err(e) => {
                                println!("Error updating message field {} {}: {}",
                                         field.name,
                                         field.value,
                                         e);
                            }
                        };
                        idx += size;
                    }
                    field.value = PprzMsgBaseType::Uint32Arr(data);
                }
                PprzMsgBaseType::Int8Arr(_) => {
                    let len = payload[idx] as usize;
                    idx += 1;

                    let mut data = vec![];
                    for k in idx..idx + len {
                        data.push(payload[k] as i8);
                        idx += 1;
                    }
                    field.value = PprzMsgBaseType::Int8Arr(data);
                }
                PprzMsgBaseType::Int16Arr(_) => {
                    let len = payload[idx] as usize; // length of the array
                    let size = mem::size_of::<i16>(); // size of one element
                    idx += 1; // increment index

                    let mut data = vec![]; // init empty vector
                    let end = idx + len * size;

                    let mut rdr = Cursor::new(&payload[idx..end]);
                    for _ in 0..len {
                        match rdr.read_i16::<LittleEndian>() {
                            Ok(v) => data.push(v),
                            Err(e) => {
                                println!("Error updating message field {} {}: {}",
                                         field.name,
                                         field.value,
                                         e);
                            }
                        };
                        idx += size;
                    }
                    field.value = PprzMsgBaseType::Int16Arr(data);
                }
                PprzMsgBaseType::Int32Arr(_) => {
                    let len = payload[idx] as usize; // length of the array
                    let size = mem::size_of::<i32>(); // size of one element
                    idx += 1; // increment index

                    let mut data = vec![]; // init empty vector
                    let end = idx + len * size;

                    let mut rdr = Cursor::new(&payload[idx..end]);
                    for _ in 0..len {
                        match rdr.read_i32::<LittleEndian>() {
                            Ok(v) => data.push(v),
                            Err(e) => {
                                println!("Error updating message field {} {}: {}",
                                         field.name,
                                         field.value,
                                         e);
                            }
                        };
                        idx += size;
                    }
                    field.value = PprzMsgBaseType::Int32Arr(data);
                }
                PprzMsgBaseType::FloatArr(_) => {
                    let len = payload[idx] as usize; // length of the array
                    let size = mem::size_of::<f32>(); // size of one element
                    idx += 1; // increment index

                    let mut data = vec![]; // init empty vector
                    let end = idx + len * size;

                    let mut rdr = Cursor::new(&payload[idx..end]);
                    for _ in 0..len {
                        match rdr.read_f32::<LittleEndian>() {
                            Ok(v) => data.push(v),
                            Err(e) => {
                                println!("Error updating message field {} {}: {}",
                                         field.name,
                                         field.value,
                                         e);
                            }
                        };
                        idx += size;
                    }
                    field.value = PprzMsgBaseType::FloatArr(data);
                }
                PprzMsgBaseType::DoubleArr(_) => {
                    let len = payload[idx] as usize; // length of the array
                    let size = mem::size_of::<f64>(); // size of one element
                    idx += 1; // increment index

                    let mut data = vec![]; // init empty vector
                    let end = idx + len * size;

                    let mut rdr = Cursor::new(&payload[idx..end]);
                    for _ in 0..len {
                        match rdr.read_f64::<LittleEndian>() {
                            Ok(v) => data.push(v),
                            Err(e) => {
                                println!("Error updating message field {} {}: {}",
                                         field.name,
                                         field.value,
                                         e);
                            }
                        };
                        idx += size;
                    }
                    field.value = PprzMsgBaseType::DoubleArr(data);
                }
               PprzMsgBaseType::String(_) => println!("Unsupported data type: String"), // String (ignore for now)
            }
        }
    }

    /// Return a string representation of the message,
    /// useful for posting the message into the IVY bus
    pub fn to_string(&self) -> Option<String> {
        let mut buf = Vec::new();
        let _ = write!(&mut buf, "{}", self);
        match String::from_utf8(buf) {
            Ok(s) => Some(s),
            Err(e) => {
                println!("Error converting msg to string: {}", e);
                return None;
            }
        }
    }

    /// Return a rexepr representing the message,
    /// useful for binding a particular message callback.
    ///
    /// The format is:
    ///
    /// * source (SENDER_ID)
    /// * message name
    /// * fields
    ///
    /// For example a `WP_MOVED` message that is defined as
    ///
    /// ```xml,ignore
    ///<message name="WP_MOVED" id="35">
    ///	<description>
    /// 	Waypoint with id wp_id has been updated/moved to the specified UTM coordinates.
    ///	</description>
    /// 	<field name="wp_id" type="uint8"/>
    ///  	<field name="utm_east" type="float" unit="m"/>
    /// 	<field name="utm_north" type="float" unit="m"/>
    ///  	<field name="alt" type="float" unit="m">Height above Mean Sea Level (geoid)</field>
    ///  	<field name="utm_zone" type="uint8"/>
    ///</message>
    /// ```
    ///
    /// will be represented as `^(\\S*) WP_MOVED (\\S*) (\\S*) (\\S*) (\\S*) (\\S*)`
    ///
    pub fn to_ivy_regexpr(&self) -> String {
        let regexpr = String::from("(\\S*)");
        let mut s: String = format!("^{} {}", regexpr, self.name);

        for field in &self.fields {
            match field.value {
                PprzMsgBaseType::Float(_) => s += &format!(" {}", regexpr),
                PprzMsgBaseType::Double(_) => s += &format!(" {}", regexpr),
                PprzMsgBaseType::Uint8(_) => s += &format!(" {}", regexpr),
                PprzMsgBaseType::Uint16(_) => s += &format!(" {}", regexpr),
                PprzMsgBaseType::Uint32(_) => s += &format!(" {}", regexpr),
                PprzMsgBaseType::Int8(_) => s += &format!(" {}", regexpr),
                PprzMsgBaseType::Int16(_) => s += &format!(" {}", regexpr),
                PprzMsgBaseType::Int32(_) => s += &format!(" {}", regexpr),
                PprzMsgBaseType::Char(_) => s += &format!(" {}", regexpr),
                PprzMsgBaseType::String(_) => s += &format!(" {}", regexpr),
                PprzMsgBaseType::FloatArr(ref v) => {
                    s += &format!("{}", regexpr);
                    for _ in 0..v.len() {
                        s += &format!(" {}", regexpr);
                    }
                }
                PprzMsgBaseType::DoubleArr(ref v) => {
                    s += &format!(" {}", regexpr);
                    for _ in 0..v.len() {
                        s += &format!(" {}", regexpr);
                    }
                }
                PprzMsgBaseType::Uint8Arr(ref v) => {
                    s += &format!(" {}", regexpr);
                    for _ in 0..v.len() {
                        s += &format!(" {}", regexpr);
                    }
                }
                PprzMsgBaseType::Uint16Arr(ref v) => {
                    s += &format!(" {}", regexpr);
                    for _ in 0..v.len() {
                        s += &format!(" {}", regexpr);
                    }
                }
                PprzMsgBaseType::Uint32Arr(ref v) => {
                    s += &format!(" {}", regexpr);
                    for _ in 0..v.len() {
                        s += &format!(" {}", regexpr);
                    }
                }
                PprzMsgBaseType::Int8Arr(ref v) => {
                    s += &format!(" {}", regexpr);
                    for _ in 0..v.len() {
                        s += &format!(" {}", regexpr);
                    }
                }
                PprzMsgBaseType::Int16Arr(ref v) => {
                    s += &format!(" {}", regexpr);
                    for _ in 0..v.len() {
                        s += &format!(" {}", regexpr);
                    }
                }
                PprzMsgBaseType::Int32Arr(ref v) => {
                    s += &format!(" {}", regexpr);
                    for _ in 0..v.len() {
                        s += &format!(" {}", regexpr);
                    }
                }
                PprzMsgBaseType::CharArr(ref v) => {
                    s += &format!(" {}", regexpr);
                    for _ in 0..v.len() {
                        s += &format!(" {}", regexpr);
                    }
                }
            }
        }
        s
    }

    /// Return a byte representation of the message and its fields,
    /// in the format of:
    ///
    /// Pprzlink 1.0
    /// ```ignore
    /// payload[0] SENDER_ID
    /// payload[1] MSG_ID
    /// payload[2-end] MSG_PAYLOAD
    /// ```
    ///
    /// Pprzlink 2.0
    /// ```ignore
    /// payload[0] source SENDER_ID
    /// payload[1] destination ID
    /// payload[2] class/component
    /// payload[3] MSG_ID
    /// payload[4-end] MSG_PAYLOAD
    /// ```
    ///
    /// Note that the byte order is LittleEndian!
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = vec![];
        
        match self.protocol {
        	PprzProtocolVersion::ProtocolV1 => {
		        buf.push(self.source); // sender ID
		        buf.push(self.id); // message ID		
        	}
        	PprzProtocolVersion::ProtocolV2 => {
        		buf.push(self.source); // sender ID
		        buf.push(self.destination); // destination ID
        		// bits 0-3: 16 class ID available
		        // bits 4-7: 16 component ID available
		        buf.push(self.class as u8 & 0xF | self.component & 0xF0); // class/component ID
		        buf.push(self.id); // message ID		
        	}
        }

        for field in &self.fields {
            match field.value {
                PprzMsgBaseType::Uint8(v) => buf.push(v),
                PprzMsgBaseType::Int8(v) => buf.push(v as u8),
                PprzMsgBaseType::Char(v) => buf.push(v as u8),
                PprzMsgBaseType::Uint16(v) => buf.write_u16::<LittleEndian>(v).unwrap(),
                PprzMsgBaseType::Int16(v) => buf.write_i16::<LittleEndian>(v).unwrap(),
                PprzMsgBaseType::Uint32(v) => buf.write_u32::<LittleEndian>(v).unwrap(),
                PprzMsgBaseType::Int32(v) => buf.write_i32::<LittleEndian>(v).unwrap(),
                PprzMsgBaseType::Float(v) => buf.write_f32::<LittleEndian>(v).unwrap(),
                PprzMsgBaseType::Double(v) => buf.write_f64::<LittleEndian>(v).unwrap(),
                PprzMsgBaseType::Uint8Arr(ref v) => {
                    buf.push(v.len() as u8);
                    for byte in v {
                        buf.push(*byte);
                    }
                }
                PprzMsgBaseType::Int8Arr(ref v) => {
                    buf.push(v.len() as u8);
                    for byte in v {
                        buf.push(*byte as u8);
                    }
                }
                PprzMsgBaseType::CharArr(ref v) => {
                    buf.push(v.len() as u8);
                    for byte in v {
                        buf.push(*byte as u8);
                    }
                }
                PprzMsgBaseType::Uint16Arr(ref v) => {
                    buf.push(v.len() as u8);
                    for byte in v {
                        buf.write_u16::<LittleEndian>(*byte).unwrap();
                    }
                }
                PprzMsgBaseType::Int16Arr(ref v) => {
                    buf.push(v.len() as u8);
                    for byte in v {
                        buf.write_i16::<LittleEndian>(*byte).unwrap();
                    }
                }
                PprzMsgBaseType::Uint32Arr(ref v) => {
                    buf.push(v.len() as u8);
                    for byte in v {
                        buf.write_u32::<LittleEndian>(*byte).unwrap();
                    }
                }
                PprzMsgBaseType::Int32Arr(ref v) => {
                    buf.push(v.len() as u8);
                    for byte in v {
                        buf.write_i32::<LittleEndian>(*byte).unwrap();
                    }
                }
                PprzMsgBaseType::FloatArr(ref v) => {
                    buf.push(v.len() as u8);
                    for byte in v {
                        buf.write_f32::<LittleEndian>(*byte).unwrap();
                    }
                }
                PprzMsgBaseType::DoubleArr(ref v) => {
                    buf.push(v.len() as u8);
                    for byte in v {
                        buf.write_f64::<LittleEndian>(*byte).unwrap();
                    }
                }
                PprzMsgBaseType::String(_) => panic!("Unsupported type: String !"), // panic for now
            }
        }
        buf
    }


    /// Set the soure (`SENDER_ID`) of the message
    /// Important when sending messages from the ground to the UAV
    pub fn set_sender(&mut self, sender_id: u8) {
        self.source = sender_id;
    }
    
    /// Set the destination (`SENDER_ID`) of the message
    pub fn set_destinaton(&mut self, destination_id: u8) {
        self.destination = destination_id;
    }
    
    /// Set class ID of the message
    /// bits 0-3: 16 class ID available
    pub fn set_class(&mut self, class_id: PprzMsgClassID) {
        self.class = class_id;
    }
    
    /// Set component ID of the message
    /// bits 4-7: 16 component ID available
    pub fn set_component(&mut self, component_id: u8) {
        self.component = component_id;
    }
    
    /// Set protocol version
    /// either Pprzlink 1.0 or Pprzlink 2.0
    pub fn set_protocol(&mut self, new_protocol: PprzProtocolVersion) {
	    self.protocol = new_protocol;
    }
}

/// Display message in IVY bus compatible format,
/// see `to_string()`
impl fmt::Display for PprzMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = format!("{} {}", self.source, self.name);

        for field in &self.fields {
            s = s + &format!(" {}", field);
        }
        write!(f, "{}", s)
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
    pub protocol: PprzProtocolVersion,
}


impl PprzDictionary {
	pub fn new(pprzlink_version: PprzProtocolVersion) -> PprzDictionary {
		PprzDictionary {
			classes: vec![],
			protocol: pprzlink_version,
		}
	}

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

    pub fn get_msgs(&self, msg_class_id: PprzMsgClassID) -> Option<PprzMsgClass> {
        for class in &self.classes {
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

pub fn build_dictionary(file: File, pprzlink_version: PprzProtocolVersion) -> PprzDictionary {
    let file = BufReader::new(file);
    let mut dictionary = PprzDictionary::new(pprzlink_version);
    let parser = EventReader::new(file);
    let mut current_class = PprzMsgClassID::Unknown;
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

                        // save the current class
                        current_class = class_name;

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
                                protocol: pprzlink_version,
                                source: 0,
                                destination: 0,
                                component: 0,
                                version: PprzMessageVersion::MessagesV1,
                                fields: vec![],
                                name: msg_name.clone(),
                                id: msg_id,
                                class: current_class,
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
