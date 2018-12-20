use std::ops::{AddAssign, MulAssign, Neg};

use serde::de::{
    self, Deserialize, DeserializeSeed, EnumAccess, IntoDeserializer,
    MapAccess, SeqAccess, VariantAccess, Visitor,
};

use std::fmt;
use std::marker::PhantomData;

use crate::error::{Error, Result};

pub struct Deserializer<'de> {
    // This string starts with the input data and characters are truncated off
    // the beginning as data is parsed.
    //input: &'de str,
    input: Vec<&'de str>,
}

impl<'de> Deserializer<'de> {
    // By convention, `Deserializer` constructors are named like `from_xyz`.
    // That way basic use cases are satisfied by something like
    // `serde_json::from_str(...)` while advanced use cases that require a
    // deserializer can make one with `serde_json::Deserializer::from_str(...)`.
    pub fn from_str(input: Vec<&'de str>) -> Self {
        Deserializer { input }
    }
}

struct SeqDeserializer<'de> {
    input: Vec<&'de str>,
}

impl <'de> SeqDeserializer<'de> {
    pub fn from_str(s: &'de str) -> Self {
        let input: Vec<&str> = s.split(&[','][..]).filter(|x| !x.is_empty()).collect();
        SeqDeserializer { input }
    }

    pub fn input_len(&self) -> usize {
        self.input.len()
    }

    fn next_element(&mut self) -> Result<&'de str> {
        println!("next_element");
        if self.input.is_empty() {
            Err(Error::Eof)
        } else {
            let val = self.input.remove(0);
            println!("val={}",val);
            Ok(val)
        }
    }

    fn parse_bool(&mut self) -> Result<bool> {
        println!("parse_bool");
        let val = self.next_element()?;
        match val {
            "0" => Ok(false),
            "1" => Ok(true),
            _ => Err(Error::ExpectedBoolean)
        }
    }


    fn parse_u8(&mut self) -> Result<u8>
    {
        println!("parse_u8");
        let val = self.next_element()?;
        println!("next elem={}",val);
        Ok(val.parse::<u8>().unwrap())
    }

    fn parse_u16(&mut self) -> Result<u16>
    {
        println!("parse_u16");
        let val = self.next_element()?;
        Ok(val.parse::<u16>().unwrap())
    }

    fn parse_u32(&mut self) -> Result<u32>
    {
        println!("parse_u32");
        let val = self.next_element()?;
        Ok(val.parse::<u32>().unwrap())
    }

    fn parse_u64(&mut self) -> Result<u64>
    {
        println!("parse_u64");
        let val = self.next_element()?;
        Ok(val.parse::<u64>().unwrap())
    }

    fn parse_i8(&mut self) -> Result<i8>
    {
        println!("parse_i8");
        let val = self.next_element()?;
        Ok(val.parse::<i8>().unwrap())
    }

    fn parse_i16(&mut self) -> Result<i16>
    {
        println!("parse_i16");
        let val = self.next_element()?;
        match val.parse::<i16>() {
            Ok(val) => Ok(val),
            Err(e) => Err(Error::Message(format!("{:?}",e)))
        }
    }

    fn parse_i32(&mut self) -> Result<i32>
    {
        println!("parse_i32");
        let val = self.next_element()?;
        Ok(val.parse::<i32>().unwrap())
    }

    fn parse_i64(&mut self) -> Result<i64>
    {
        println!("parse_i64");
        let val = self.next_element()?;
        Ok(val.parse::<i64>().unwrap())
    }

    fn parse_f32(&mut self) -> Result<f32>
    {
        println!("parse_f32");
        let val = self.next_element()?;
        Ok(val.parse::<f32>().unwrap())
    }

    fn parse_f64(&mut self) -> Result<f64>
    {
        println!("parse_f64");
        let val = self.next_element()?;
        Ok(val.parse::<f64>().unwrap())
    }

    fn parse_string(&mut self) -> Result<&'de str> {
        println!("parse_string");
        self.next_element()
    }

    fn parse_char(&mut self) -> Result<char> {
        println!("parse_string");
        unimplemented!();
    }
}


// By convention, the public API of a Serde deserializer is one or more
// `from_xyz` methods such as `from_str`, `from_bytes`, or `from_reader`
// depending on what Rust types the deserializer is able to consume as input.
//
// This basic deserializer supports only `from_str`.
pub fn from_str<'a, T>(s: &'a str) -> Result<T>
where
    T: Deserialize<'a>,
{
    println!("from_str");
    // TODO: split on space only to separate sequences
    //let mut input: Vec<&str> = s.split(&[' ',','][..]).filter(|x| !x.is_empty()).collect();
    let input: Vec<&str> = s.split(&[' '][..]).filter(|x| !x.is_empty()).collect();

    //let sender = input.remove(0);
    println!("Input: {:?}", input);

    let mut deserializer = Deserializer::from_str(input);
    let t = T::deserialize(&mut deserializer)?;
    if deserializer.input.is_empty() {
        Ok(t)
    } else {
        Err(Error::TrailingCharacters)
    }
}

impl<'de> Deserializer<'de> {
    fn next_element(&mut self) -> Result<&'de str> {
        println!("next_element");
        if self.input.is_empty() {
            Err(Error::Eof)
        } else {
            let val = self.input.remove(0);
            println!("val={}",val);
            Ok(val)
        }
    }

    fn parse_bool(&mut self) -> Result<bool> {
        println!("parse_bool");
        let val = self.next_element()?;
        match val {
            "0" => Ok(false),
            "1" => Ok(true),
            _ => Err(Error::ExpectedBoolean)
        }
    }


    fn parse_u8(&mut self) -> Result<u8>
    {
        println!("parse_u8");
        let val = self.next_element()?;
        println!("next elem={}",val);
        match val.parse::<u8>() {
            Ok(val) => Ok(val),
            Err(e) => {
                // handle this better
                Ok(val.as_bytes()[0])
            }
        }
    }

    fn parse_u16(&mut self) -> Result<u16>
    {
        println!("parse_u16");
        let val = self.next_element()?;
        Ok(val.parse::<u16>().unwrap())
    }

    fn parse_u32(&mut self) -> Result<u32>
    {
        println!("parse_u32");
        let val = self.next_element()?;
        Ok(val.parse::<u32>().unwrap())
    }

    fn parse_u64(&mut self) -> Result<u64>
    {
        println!("parse_u64");
        let val = self.next_element()?;
        Ok(val.parse::<u64>().unwrap())
    }

    fn parse_i8(&mut self) -> Result<i8>
    {
        println!("parse_i8");
        let val = self.next_element()?;
        Ok(val.parse::<i8>().unwrap())
    }

    fn parse_i16(&mut self) -> Result<i16>
    {
        println!("parse_i16");
        let val = self.next_element()?;
        Ok(val.parse::<i16>().unwrap())
    }

    fn parse_i32(&mut self) -> Result<i32>
    {
        println!("parse_i32");
        let val = self.next_element()?;
        Ok(val.parse::<i32>().unwrap())
    }

    fn parse_i64(&mut self) -> Result<i64>
    {
        println!("parse_i64");
        let val = self.next_element()?;
        Ok(val.parse::<i64>().unwrap())
    }

    fn parse_f32(&mut self) -> Result<f32>
    {
        println!("parse_f32");
        let val = self.next_element()?;
        Ok(val.parse::<f32>().unwrap())
    }

    fn parse_f64(&mut self) -> Result<f64>
    {
        println!("parse_f64");
        let val = self.next_element()?;
        Ok(val.parse::<f64>().unwrap())
    }

    fn parse_string(&mut self) -> Result<&'de str> {
        println!("parse_string");
        self.next_element()
    }

    fn parse_char(&mut self) -> Result<char> {
        println!("parse_string");
        unimplemented!();
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut SeqDeserializer<'de> {
    type Error = Error;

   fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {   
        println!("deserialize_any");
        unimplemented!();
    }

    // Uses the `parse_bool` parsing function defined above to read the JSON
    // identifier `true` or `false` from the input.
    //
    // Parsing refers to looking at the input and deciding that it contains the
    // JSON value `true` or `false`.
    //
    // Deserialization refers to mapping that JSON value into Serde's data
    // model by invoking one of the `Visitor` methods. In the case of JSON and
    // bool that mapping is straightforward so the distinction may seem silly,
    // but in other cases Deserializers sometimes perform non-obvious mappings.
    // For example the TOML format has a Datetime type and Serde's data model
    // does not. In the `toml` crate, a Datetime in the input is deserialized by
    // mapping it to a Serde data model "struct" type with a special name and a
    // single field containing the Datetime represented as a string.
    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        println!("deserialize_bool");
        visitor.visit_bool(self.parse_bool()?)
    }

    // The `parse_signed` function is generic over the integer type `T` so here
    // it is invoked with `T=i8`. The next 8 methods are similar.
    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(self.parse_i8()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(self.parse_i16()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(self.parse_i32()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(self.parse_i64()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(self.parse_u8()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(self.parse_u16()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(self.parse_u32()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.parse_u64()?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f32(self.parse_f32()?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f64(self.parse_f64()?)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_char(self.parse_char()?)
    }

    // Refer to the "Understanding deserializer lifetimes" page for information
    // about the three deserialization flavors of strings in Serde.
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        println!("deserialize_str");
        visitor.visit_borrowed_str(self.parse_string()?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        println!("deserialize_string");
        self.deserialize_str(visitor)
    }

    // The `Serializer` implementation on the previous page serialized byte
    // arrays as JSON arrays of bytes. Handle that representation here.
    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        println!("deserialize_bytes");
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        println!("deserialize_byte_buf");
        unimplemented!()
    }

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        println!("deserialize_option");
        unimplemented!();
    }

    // In Serde, unit means an anonymous value containing no data.
    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        println!("deserialize_unit");
        unimplemented!();
    }

    // Unit struct means a named value containing no data.
    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        println!("deserialize_unit_struct");
        //self.deserialize_unit(visitor)
        unimplemented!();
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        println!("deserialize_newtype_struct");
        //visitor.visit_newtype_struct(self)
        unimplemented!();
    }

    // Deserialization of compound types like sequences and maps happens by
    // passing the visitor an "Access" object that gives it the ability to
    // iterate through the data contained in the sequence.
    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        println!("deserialize_seq");
        //unimplemented!();
        //let de =  SeqDeserializer::from_str(self.next_element().unwrap());
        //de.deserialize_seq(visitor)
        visitor.visit_seq(BasicSeqAccess2::new(self))
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        println!("deserialize_tuple");
        //self.deserialize_seq(visitor)
        unimplemented!();
    }

    // Tuple structs look just like sequences in JSON.
    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        println!("deserialize_tuple_struct");
        //self.deserialize_seq(visitor)
        unimplemented!();
    }

    // Much like `deserialize_seq` but calls the visitors `visit_map` method
    // with a `MapAccess` implementation, rather than the visitor's `visit_seq`
    // method with a `SeqAccess` implementation.
    fn deserialize_map<V>(mut self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        println!("deserialize_map");
        //let value = visitor.visit_map(CommaSeparated::new(&mut self, vec![]))?;
        //Ok(value)
        unimplemented!();
    }

    // Structs look just like maps in JSON.
    //
    // Notice the `fields` parameter - a "struct" in the Serde data model means
    // that the `Deserialize` implementation is required to know what the fields
    // are before even looking at the input data. Any key-value pairing in which
    // the fields cannot be known ahead of time is probably a map.
    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V, 
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        println!("deserialize_struct, name={}, fields={:?}", _name, _fields);
        //visitor.visit_seq(BasicSeqAccess::new(&mut self))
        unimplemented!();
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        println!("deserialize_enum, name={}, variants={:?}", _name, _variants);
        //let value = visitor.visit_enum(Enum::new(self))?;
        //Ok(value)
        unimplemented!();
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        println!("deserialize_identifier");
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        println!("deserialize_ignored_any");
        //self.deserialize_any(visitor)
        unimplemented!()
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {   
        println!("deserialize_any");
        unimplemented!();
    }

    // Uses the `parse_bool` parsing function defined above to read the JSON
    // identifier `true` or `false` from the input.
    //
    // Parsing refers to looking at the input and deciding that it contains the
    // JSON value `true` or `false`.
    //
    // Deserialization refers to mapping that JSON value into Serde's data
    // model by invoking one of the `Visitor` methods. In the case of JSON and
    // bool that mapping is straightforward so the distinction may seem silly,
    // but in other cases Deserializers sometimes perform non-obvious mappings.
    // For example the TOML format has a Datetime type and Serde's data model
    // does not. In the `toml` crate, a Datetime in the input is deserialized by
    // mapping it to a Serde data model "struct" type with a special name and a
    // single field containing the Datetime represented as a string.
    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        println!("deserialize_bool");
        visitor.visit_bool(self.parse_bool()?)
    }

    // The `parse_signed` function is generic over the integer type `T` so here
    // it is invoked with `T=i8`. The next 8 methods are similar.
    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(self.parse_i8()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(self.parse_i16()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(self.parse_i32()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(self.parse_i64()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(self.parse_u8()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(self.parse_u16()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(self.parse_u32()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.parse_u64()?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f32(self.parse_f32()?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f64(self.parse_f64()?)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_char(self.parse_char()?)
    }

    // Refer to the "Understanding deserializer lifetimes" page for information
    // about the three deserialization flavors of strings in Serde.
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        println!("deserialize_str");
        visitor.visit_borrowed_str(self.parse_string()?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        println!("deserialize_string");
        self.deserialize_str(visitor)
    }

    // The `Serializer` implementation on the previous page serialized byte
    // arrays as JSON arrays of bytes. Handle that representation here.
    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        println!("deserialize_bytes");
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        println!("deserialize_byte_buf");
        unimplemented!()
    }

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        println!("deserialize_option");
        unimplemented!();
    }

    // In Serde, unit means an anonymous value containing no data.
    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        println!("deserialize_unit");
        unimplemented!();
    }

    // Unit struct means a named value containing no data.
    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        println!("deserialize_unit_struct");
        //self.deserialize_unit(visitor)
        unimplemented!();
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        println!("deserialize_newtype_struct");
        //visitor.visit_newtype_struct(self)
        unimplemented!();
    }

    // Deserialization of compound types like sequences and maps happens by
    // passing the visitor an "Access" object that gives it the ability to
    // iterate through the data contained in the sequence.
    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        println!("deserialize_seq");
        //unimplemented!();
        let mut de =  SeqDeserializer::from_str(self.next_element().unwrap());
        de.deserialize_seq(visitor)

    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        println!("deserialize_tuple");
        //self.deserialize_seq(visitor)
        unimplemented!();
    }

    // Tuple structs look just like sequences in JSON.
    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        println!("deserialize_tuple_struct");
        //self.deserialize_seq(visitor)
        unimplemented!();
    }

    // Much like `deserialize_seq` but calls the visitors `visit_map` method
    // with a `MapAccess` implementation, rather than the visitor's `visit_seq`
    // method with a `SeqAccess` implementation.
    fn deserialize_map<V>(mut self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        println!("deserialize_map");
        //let value = visitor.visit_map(CommaSeparated::new(&mut self, vec![]))?;
        //Ok(value)
        unimplemented!();
    }

    // Structs look just like maps in JSON.
    //
    // Notice the `fields` parameter - a "struct" in the Serde data model means
    // that the `Deserialize` implementation is required to know what the fields
    // are before even looking at the input data. Any key-value pairing in which
    // the fields cannot be known ahead of time is probably a map.
    fn deserialize_struct<V>(
        mut self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V, 
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        println!("deserialize_struct, name={}, fields={:?}", _name, _fields);
        visitor.visit_seq(BasicSeqAccess::new(&mut self))
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        println!("deserialize_enum, name={}, variants={:?}", _name, _variants);
        let value = visitor.visit_enum(Enum::new(self))?;
        Ok(value)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        println!("deserialize_identifier");
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        println!("deserialize_ignored_any");
        //self.deserialize_any(visitor)
        unimplemented!()
    }
}



/*
/// To deserialize arrays
struct CommaSeparatedAccess<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> CommaSeparatedAccess<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>, values: Vec<&'a str>) -> Self {
        println!("new");
        CommaSeparatedAccess {
            de,
        }
    }
}

// `SeqAccess` is provided to the `Visitor` to give it the ability to iterate
// through elements of the sequence.
impl<'de, 'a> SeqAccess<'de> for CommaSeparatedAccess<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        println!("next_element_seed");
        
        seed.deserialize(&mut *self.de).map(Some)
    }
}
*/

/// To deserialize structs
struct BasicSeqAccess<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> BasicSeqAccess<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        println!("new");
        BasicSeqAccess {
            de,
        }
    }
}

// `SeqAccess` is provided to the `Visitor` to give it the ability to iterate
// through elements of the sequence.
impl<'de, 'a> SeqAccess<'de> for BasicSeqAccess<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        println!("next_element_seed");
        seed.deserialize(&mut *self.de).map(Some)
    }
}

/// To deserialize structs
struct BasicSeqAccess2<'a, 'de: 'a> {
    de: &'a mut SeqDeserializer<'de>,
}

impl<'a, 'de> BasicSeqAccess2<'a, 'de> {
    fn new(de: &'a mut SeqDeserializer<'de>) -> Self {
        println!("new");
        BasicSeqAccess2 {
            de,
        }
    }
}

// `SeqAccess` is provided to the `Visitor` to give it the ability to iterate
// through elements of the sequence.
impl<'de, 'a> SeqAccess<'de> for BasicSeqAccess2<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        println!("next_element_seed");
        if self.de.input_len() > 0 {
            seed.deserialize(&mut *self.de).map(Some)
        } else {
            Ok(None)
        }
    }
}

/*
// `MapAccess` is provided to the `Visitor` to give it the ability to iterate
// through entries of the map.
impl<'de, 'a> MapAccess<'de> for BasicSeqAccess<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        println!("next_key_seed");
        //unimplemented!();
        seed.deserialize(&mut *self.de).map(Some)
        //Ok(None)
        /*
        // Check if there are no more entries.
        if self.de.peek_char()? == '}' {
            return Ok(None);
        }
        // Comma is required before every entry except the first.
        if !self.first && self.de.next_char()? != ',' {
            return Err(Error::ExpectedMapComma);
        }
        self.first = false;
        // Deserialize a map key.
        seed.deserialize(&mut *self.de).map(Some)
        */
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        println!("next_value_seed");
        seed.deserialize(&mut *self.de)
        //unimplemented!();
        /*
        // It doesn't make a difference whether the colon is parsed at the end
        // of `next_key_seed` or at the beginning of `next_value_seed`. In this
        // case the code is a bit simpler having it here.
        if self.de.next_char()? != ':' {
            return Err(Error::ExpectedMapColon);
        }
        // Deserialize a map value.
        seed.deserialize(&mut *self.de)
        */
    }
}
*/
struct Enum<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> Enum<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        println!("Enum_new");
        Enum { de }
    }
}

// `EnumAccess` is provided to the `Visitor` to give it the ability to determine
// which variant of the enum is supposed to be deserialized.
//
// Note that all enum deserialization methods in Serde refer exclusively to the
// "externally tagged" enum representation.
impl<'de, 'a> EnumAccess<'de> for Enum<'a, 'de> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: DeserializeSeed<'de>,
    {
        println!("variant_seed");
        let val = seed.deserialize(&mut *self.de)?;
        Ok((val, self))
        /*
        // The `deserialize_enum` method parsed a `{` character so we are
        // currently inside of a map. The seed will be deserializing itself from
        // the key of the map.
        let val = seed.deserialize(&mut *self.de)?;
        // Parse the colon separating map key from value.
        if self.de.next_char()? == ':' {
            Ok((val, self))
        } else {
            Err(Error::ExpectedMapColon)
        }
        */
    }
}

// `VariantAccess` is provided to the `Visitor` to give it the ability to see
// the content of the single variant that it decided to deserialize.
impl<'de, 'a> VariantAccess<'de> for Enum<'a, 'de> {
    type Error = Error;

    // If the `Visitor` expected this variant to be a unit variant, the input
    // should have been the plain string case handled in `deserialize_enum`.
    fn unit_variant(self) -> Result<()> {
        println!("unit_variant");
        Err(Error::ExpectedString)
    }

    // Newtype variants are represented in JSON as `{ NAME: VALUE }` so
    // deserialize the value here.
    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: DeserializeSeed<'de>,
    {
        println!("newtype+variant_seed");
        seed.deserialize(self.de)
    }

    // Tuple variants are represented in JSON as `{ NAME: [DATA...] }` so
    // deserialize the sequence of data here.
    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        println!("tuple_variant");
        de::Deserializer::deserialize_seq(self.de, visitor)
    }

    // Struct variants are represented in JSON as `{ NAME: { K: V, ... } }` so
    // deserialize the inner map here.
    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        println!("struct_variant");
        de::Deserializer::deserialize_map(self.de, visitor)
    }
}
