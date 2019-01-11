#![cfg_attr(not(feature = "std"), feature(alloc))]
#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::fmt;
#[cfg(not(feature = "std"))]
use alloc::string::String;

#[cfg(feature = "std")]
use std::fmt;

#[cfg(feature = "serde-derive")]
#[macro_use]
#[cfg(feature = "serde-derive")]
extern crate serde_derive;
#[cfg(feature = "serde-derive")]
extern crate serde;

// Pprzlink message set
// Note: the unused code will be optimized away
// TODO: make a macro?
include!(concat!(env!("OUT_DIR"), "/telemetry.rs"));
include!(concat!(env!("OUT_DIR"), "/ground.rs"));
include!(concat!(env!("OUT_DIR"), "/datalink.rs"));
include!(concat!(env!("OUT_DIR"), "/alert.rs"));
include!(concat!(env!("OUT_DIR"), "/intermcu.rs"));

pub use self::alert::PprzMessageAlert;
pub use self::datalink::PprzMessageDatalink;
pub use self::ground::PprzMessageGround;
pub use self::intermcu::PprzMessageIntermcu;
pub use self::telemetry::PprzMessageTelemetry;

/// Enum encapsulating all message types
#[derive(Clone, PartialEq, Debug)]
pub enum PprzMessage {
    Telemetry(PprzMessageTelemetry),
    Ground(PprzMessageGround),
    Datalink(PprzMessageDatalink),
    Alert(PprzMessageAlert),
    Intermcu(PprzMessageIntermcu),
}

impl fmt::Display for PprzMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::PprzMessage::*;
        match self {
            Telemetry(m) => m.fmt(f),
            Ground(m) => m.fmt(f),
            Datalink(m) => m.fmt(f),
            Alert(m) => m.fmt(f),
            Intermcu(m) => m.fmt(f),
        }
    }
}

impl PprzMessage {
    pub fn create_ivy_message(&self, sender: &str) -> String {
        use self::PprzMessage::*;
        match self {
            Telemetry(m) => m.create_ivy_message(sender),
            Ground(m) => m.create_ivy_message(sender),
            Datalink(m) => m.create_ivy_message(sender),
            Alert(m) => m.create_ivy_message(sender),
            Intermcu(m) => m.create_ivy_message(sender),
        }
    }

    pub fn parse_ivy_msg_from_sender(input: &str, sender: Option<&str>) -> Option<PprzMessage> {
        let mut input = input.chars();
        let parsed_sender: String = input.by_ref().take_while(|x| *x != ' ').collect();
        if let Some(expected_sender) = sender {
            if parsed_sender != expected_sender {
                return None;
            }
        }
        PprzMessage::from_str(&mut input.as_str())
    }

    pub fn from_str(s: &str) -> Option<PprzMessage> {
        if let Some(msg) = PprzMessageTelemetry::from_str(s) {
            return Some(PprzMessage::Telemetry(msg));
        }
        if let Some(msg) = PprzMessageGround::from_str(s) {
            return Some(PprzMessage::Ground(msg));
        }
        if let Some(msg) = PprzMessageDatalink::from_str(s) {
            return Some(PprzMessage::Datalink(msg));
        }
        if let Some(msg) = PprzMessageAlert::from_str(s) {
            return Some(PprzMessage::Alert(msg));
        }
        if let Some(msg) = PprzMessageIntermcu::from_str(s) {
            return Some(PprzMessage::Intermcu(msg));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::PprzMessage;
    #[cfg(feature = "std")]
    use crate::datalink;
    #[cfg(feature = "std")]
    use crate::ground;
    #[cfg(feature = "std")]
    use crate::telemetry;
    #[cfg(feature = "std")]
    use std::io::BufRead;

    #[cfg(feature = "test-serde")]
    use serde_json;

    // dummy test for no-std testing
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    /// Simple test with Serde to JSON format
    /// Note: this seems to take a lot of time to compile, the performance is unknown
    #[cfg(feature = "test-serde")]
    #[test]
    fn test_serde() {
        let m = telemetry::PprzMessageTelemetry::NPS_RATE_ATTITUDE(
            telemetry::NPS_RATE_ATTITUDE_DATA::default(),
        );
        let s = serde_json::to_string(&m).unwrap();
        println!("{}", s);
    }

    #[cfg(feature = "std")]
    #[test]
    fn test1() {
        let m = telemetry::PprzMessageTelemetry::NPS_RATE_ATTITUDE(
            telemetry::NPS_RATE_ATTITUDE_DATA::default(),
        );
        let s = m.to_string();
        println!("{}", s);
        let msg = telemetry::PprzMessageTelemetry::from_str(&s);
        println!("{:#?}", msg);
    }

    /// Test parsing for a sender
    #[cfg(feature = "std")]
    #[test]
    fn test_sender() {
        let input = "42 GPS_LLA 418155620 -1119824356 1989372 1349981 901 1 0 1794 10805 3 0";
        let res = PprzMessage::parse_ivy_msg_from_sender(input, Some("42"));
        assert_ne!(res, None);
        let msg = res.unwrap();
        let res = PprzMessage::create_ivy_message(&msg, "42");
        println!("res={}", res);
    }

    // test ser and deser
    #[cfg(feature = "std")]
    #[test]
    fn test_serial() {
        let f = std::fs::File::open("./test.txt").unwrap();
        let mut cnt = 0;
        for line in std::io::BufReader::new(f).lines() {
            cnt = cnt + 1;
            let l = line.unwrap();
            let mut input: Vec<&str> = l.split(&[' '][..]).collect();
            input.remove(0);
            let l: String = input.iter().map(|a| String::from(*a) + " ").collect();
            let mut l: Vec<char> = l.chars().collect();
            l.pop();
            let l: String = l.iter().collect();

            println!(">>>>>>>>>>>>>>>>>>");
            println!("Line # {}= >{}<", cnt, l);
            let msg = telemetry::PprzMessageTelemetry::from_str(&l);
            match msg {
                Some(m) => {
                    println!("line {}, telemetry:, {:#?}", cnt, m);
                    let s = m.ser();
                    println!("vec={:?}", s);
                    let v = telemetry::PprzMessageTelemetry::deser(&s).unwrap();
                    println!("{:#?}", v);
                    println!("<<<<<<<<<<<<<<<");
                    continue;
                }
                None => {
                    println!("Not telemetry msg:-( ");
                }
            }

            let msg = ground::PprzMessageGround::from_str(&l);
            match msg {
                Some(m) => {
                    println!("line {}, ground:, {:#?}", cnt, m);
                    let s = m.ser();
                    println!("vec={:?}", s);
                    let v = ground::PprzMessageGround::deser(&s).unwrap();
                    println!("{:#?}", v);
                    println!("<<<<<<<<<<<<<<<");
                    continue;
                }
                None => {
                    println!("Not ground msg:-( ");
                }
            }

            let msg = datalink::PprzMessageDatalink::from_str(&l);
            match msg {
                Some(m) => {
                    println!("line {}, datalink:, {:#?}", cnt, m);
                    let s = m.ser();
                    println!("vec={:?}", s);
                    let v = datalink::PprzMessageDatalink::deser(&s).unwrap();
                    println!("{:#?}", v);
                    println!("<<<<<<<<<<<<<<<");
                    continue;
                }
                None => {
                    println!("Not datalink msg:-(");
                }
            }

            panic!("Unrecognized input: {}", l);
        }
    }

    // test from_str and to_str
    #[cfg(feature = "std")]
    #[test]
    fn test_str() {
        let f = std::fs::File::open("./test.txt").unwrap();
        let mut cnt = 0;
        for line in std::io::BufReader::new(f).lines() {
            cnt = cnt + 1;
            let l = line.unwrap();
            let mut input: Vec<&str> = l.split(&[' '][..]).collect();
            input.remove(0);
            let l: String = input.iter().map(|a| String::from(*a) + " ").collect();
            let mut l: Vec<char> = l.chars().collect();
            l.pop();
            let l: String = l.iter().collect();

            println!(">>>>>>>>>>>>>>>>>>");
            println!("Line # {}= >{}<", cnt, l);
            let msg = telemetry::PprzMessageTelemetry::from_str(&l);
            match msg {
                Some(m) => {
                    println!("line {}, telemetry:, {:#?}", cnt, m);
                    let s = m.to_string();
                    println!("line to string: {}", s);
                    let v = telemetry::PprzMessageTelemetry::from_str(&s).unwrap();
                    println!("{:#?}", v);
                    println!("<<<<<<<<<<<<<<<");
                    continue;
                }
                None => {
                    println!("Not telemetry msg:-( ");
                }
            }

            let msg = ground::PprzMessageGround::from_str(&l);
            match msg {
                Some(m) => {
                    println!("line {}, ground:, {:#?}", cnt, m);
                    let s = m.to_string();
                    println!("line to string: {}", s);
                    let v = ground::PprzMessageGround::from_str(&s).unwrap();
                    println!("{:#?}", v);
                    println!("<<<<<<<<<<<<<<<");
                    continue;
                }
                None => {
                    println!("Not ground msg:-( ");
                }
            }

            let msg = datalink::PprzMessageDatalink::from_str(&l);
            match msg {
                Some(m) => {
                    println!("line {}, datalink:, {:#?}", cnt, m);
                    let s = m.to_string();
                    println!("line to string: {}", s);
                    let v = datalink::PprzMessageDatalink::from_str(&s).unwrap();
                    println!("{:#?}", v);
                    println!("<<<<<<<<<<<<<<<");
                    continue;
                }
                None => {
                    println!("Not datalink msg:-(");
                }
            }

            panic!("Unrecognized input: {}", l);
        }
    }

}
