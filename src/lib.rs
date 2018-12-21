//#[macro_use]
//extern crate serde_derive;
//extern crate serde;


/// Pprzlink message set
// TODO: make a macro?
include!(concat!(env!("OUT_DIR"), "/telemetry.rs"));
include!(concat!(env!("OUT_DIR"), "/ground.rs"));
include!(concat!(env!("OUT_DIR"), "/datalink.rs"));
include!(concat!(env!("OUT_DIR"), "/alert.rs"));
include!(concat!(env!("OUT_DIR"), "/intermcu.rs"));



mod test {
    use crate::telemetry;
    use crate::ground;
    use crate::datalink;

    use std::fs::File;
    use std::io::{BufRead, BufReader};

    #[test]
    fn test1() {
        let m = telemetry::PprzMessageTelemetry::NPS_RATE_ATTITUDE(telemetry::NPS_RATE_ATTITUDE_DATA::default());
        //let m = telemetry::NPS_RATE_ATTITUDE_DATA::default();
        let s = m.to_string();
        println!("{}", s);
        let msg = telemetry::PprzMessageTelemetry::from_str(&s);
        println!("{:#?}", msg);
    }

    #[test]
    fn big_test() {
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
        let msg = telemetry::PprzMessageTelemetry::from_str(&l);
        match msg {
            Some(m) => {
                println!("line {}, telemetry:, {:#?}", cnt, m);
                let s = m.to_string();
                println!("line to string: {}",s);
                let v =  telemetry::PprzMessageTelemetry::from_str(&s).unwrap();
                println!("{:#?}", v);
                println!("<<<<<<<<<<<<<<<");
                continue;
            }
            None => {
                println!("Not telemetry msg:-( ");
            }
        }

        let msg =ground::PprzMessageGround::from_str(&l);
        match msg {
            Some(m) => {
                println!("line {}, ground:, {:#?}", cnt, m);
                let s = m.to_string();
                println!("line to string: {}",s);
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
                println!("line to string: {}",s);
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

/*
struct ExtendVec<T>(Vec<T>);

impl std::fmt::Display for ExtendVec<f32> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = String::new();
        for val in &self.0 {
            s += &val.to_string();
            s += ",";
        }
        write!(f, "{}", s)
    }
}

impl<T> Into<Vec<T>> for ExtendVec<T> {
    fn into(self) -> Vec<T> {
        self.0
    }
}

impl<T> From<Vec<T>> for ExtendVec<T> {
    fn from(v: Vec<T>) -> ExtendVec<T> {
        ExtendVec(v)
    }
}
*/

// TODO: fix parsing arrays
//fn main() {  
    //let m = telemetry::PprzMessageTelemetry::NPS_RATE_ATTITUDE(telemetry::NPS_RATE_ATTITUDE_DATA::default());
    //let s = m.to_string();
    //println!("{:#?}", m);

    /*
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
    */
//}
