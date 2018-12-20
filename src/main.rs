#[macro_use]
extern crate serde_derive;
extern crate serde;

// serde custom data format
pub mod de;
pub mod error;
pub mod ser;

use crate::de::{from_str, Deserializer};
use crate::error::{Error, Result};
use crate::ser::{to_string, Serializer};


use std::fs::File;
use std::io::{BufRead, BufReader};
/// Pprzlink message set
include!("../telemetry.rs");
include!("../ground.rs");
include!("../datalink.rs");

// TODO: fix parsing arrays
fn main() {
    /*
    let s = "42 COMMANDS 0,2,1696,0,0";
    let mut input: Vec<&str> = s.split(&[' '][..]).collect();
    input.remove(0);
    let s: String = input.iter().map(|a| String::from(*a) + " ").collect();

    let new_msg: telemetry::PprzMessageTelemetry = from_str(&s).unwrap();
    println!("{:#?}",new_msg);
*/
    
    let f = File::open("./test.txt").unwrap();
    let mut cnt = 0;
    for line in BufReader::new(f).lines() {
        cnt = cnt + 1;
        let l = line.unwrap();
        let mut input: Vec<&str> = l.split(&[' '][..]).collect();
        input.remove(0);
        let l: String = input.iter().map(|a| String::from(*a) + " ").collect();
        let mut l: Vec<char> = l.chars().collect();
        l.pop();
        let l: String = l.iter().collect();

        println!(">>>>>>>>>>>>>>>>>>");
        println!("Line # {}= >{}<",cnt,l);
        let msg: Result<telemetry::PprzMessageTelemetry> = from_str(&l);
        match msg {
            Ok(m) => {
                println!("line {}, telemetry:, {:#?}", cnt, m);
                let s = to_string(&m).unwrap();
                println!("line to string: {}",s);
                let v: telemetry::PprzMessageTelemetry = from_str(&s).unwrap();
                println!("{:#?}", v);
                println!("<<<<<<<<<<<<<<<");
                continue;
            }
            Err(e) => {
                println!("Not telemetry msg:-( Err: {:?}",e);
            }
        }

        let msg: Result<ground::PprzMessageGround> = from_str(&l);
        match msg {
            Ok(m) => {
                println!("line {}, ground:, {:#?}", cnt, m);
                let s = to_string(&m).unwrap();
                println!("line to string: {}",s);
                let v: ground::PprzMessageGround = from_str(&s).unwrap();
                println!("{:#?}", v);
                println!("<<<<<<<<<<<<<<<");
                continue;
            }
            Err(e) => {
                println!("Not ground msg:-( Err: {:?}",e);
            }
        }

        let msg: Result<datalink::PprzMessageDatalink> = from_str(&l);
        match msg {
            Ok(m) => {
                println!("line {}, datalink:, {:#?}", cnt, m);
                let s = to_string(&m).unwrap();
                println!("line to string: {}",s);
                let v: datalink::PprzMessageDatalink = from_str(&s).unwrap();
                println!("{:#?}", v);
                println!("<<<<<<<<<<<<<<<");
                continue;
            }
            Err(e) => {
                println!("Not datalink msg:-( Err: {:?}",e);
            }
        }

        panic!("Unrecognized input: {}", l);
    }
    
    /*
    let m = telemetry::PprzMessageTelemetry::NPS_RATE_ATTITUDE(telemetry::NPS_RATE_ATTITUDE_DATA::default());
    let s = to_string(&m).unwrap();
    println!("{}", s);

    let m = telemetry::PprzMessageTelemetry::RTOS_MON(telemetry::RTOS_MON_DATA::default());
    let s = to_string(&m).unwrap();
    println!("{}", s);

    let m = telemetry::PprzMessageTelemetry::DL_VALUE(telemetry::DL_VALUE_DATA::default());
    let s = to_string(&m).unwrap();
    println!("{}", s);
    
        /*
    let m = telemetry::PprzMessageTelemetry::GPS_LLA(telemetry::GPS_LLA_DATA::default());
    let s = to_string(&m).unwrap();
    println!("{}", s);
    */

    //let new_m: telemetry::DL_VALUE_DATA = from_str(&s).unwrap(); // This is an easier problem
    //let new_m: telemetry::PprzMessageTelemetry = from_str(&s).unwrap();
    //let new_m: telemetry::PprzMessageTelemetry = from_str("NPS_POS_LLH 0.729819 0.729814 0.726483 -1.954462 -1.954462 1989.372587 10811.875643 -0.017408 1349.982594").unwrap();
    let new_m: telemetry::PprzMessageTelemetry = from_str("TELEMETRY_STATUS 42 -1 0.054820 15743 474 1027.0 5 5 2 0 995 1.74").unwrap();
    println!("{:#?}", new_m);
    */
}
