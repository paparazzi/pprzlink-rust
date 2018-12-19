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

/// Pprzlink message set
include!("../telemetry.rs");

fn main() {
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

    let new_m: telemetry::DL_VALUE_DATA = from_str(&s).unwrap(); // This is an easier problem
    //let new_m: telemetry::PprzMessageTelemetry = from_str(&s).unwrap();
    println!("{:#?}", new_m);
}
