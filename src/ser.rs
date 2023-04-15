use alloc::{string::String, vec::Vec};
use core::{fmt::Display, num::FpCategory};

use serde::ser::{self, Serialize};

use crate::never::Never;

pub struct Serializer {
    buf: String,
}

pub struct SeqSerializer<'a> {
    ser: &'a mut Serializer,
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Never;
    type SerializeSeq = SeqSerializer<'a>;
    type SerializeTuple = SeqSerializer<'a>;
    type SerializeTupleStruct = SeqSerializer<'a>;
    type SerializeTupleVariant = SeqSerializer<'a>;
    type SerializeMap = SeqSerializer<'a>;
    type SerializeStruct = SeqSerializer<'a>;
    type SerializeStructVariant = SeqSerializer<'a>;

    fn serialize_bool(self, v: bool) -> Result<(), Self::Error> {
        if v {
            self.buf.push_str("!t");
        } else {
            self.buf.push_str("!f");
        }
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<(), Self::Error> {
        int_to_string(&mut self.buf, v);
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<(), Self::Error> {
        int_to_string(&mut self.buf, v);
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<(), Self::Error> {
        int_to_string(&mut self.buf, v);
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<(), Self::Error> {
        int_to_string(&mut self.buf, v);
        Ok(())
    }

    fn serialize_i128(self, v: i128) -> Result<(), Self::Error> {
        int_to_string(&mut self.buf, v);
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<(), Self::Error> {
        int_to_string(&mut self.buf, v);
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<(), Self::Error> {
        int_to_string(&mut self.buf, v);
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<(), Self::Error> {
        int_to_string(&mut self.buf, v);
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<(), Self::Error> {
        int_to_string(&mut self.buf, v);
        Ok(())
    }

    fn serialize_u128(self, v: u128) -> Result<(), Self::Error> {
        int_to_string(&mut self.buf, v);
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<(), Self::Error> {
        match v.classify() {
            FpCategory::Nan | FpCategory::Infinite => self.serialize_unit(),
            _ => {
                float_to_string(&mut self.buf, v);
                Ok(())
            }
        }
    }

    fn serialize_f64(self, v: f64) -> Result<(), Self::Error> {
        match v.classify() {
            FpCategory::Nan | FpCategory::Infinite => self.serialize_unit(),
            _ => {
                float_to_string(&mut self.buf, v);
                Ok(())
            }
        }
    }

    fn serialize_char(self, v: char) -> Result<(), Self::Error> {
        todo!()
    }

    fn serialize_str(self, v: &str) -> Result<(), Self::Error> {
        todo!()
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<(), Self::Error> {
        todo!()
    }

    fn serialize_none(self) -> Result<(), Self::Error> {
        self.serialize_unit()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_unit(self) -> Result<(), Self::Error> {
        self.buf.push_str("!n");
        Ok(())
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<(), Self::Error> {
        todo!()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        todo!()
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        todo!()
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        todo!()
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }

    fn collect_str<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Display,
    {
        todo!()
    }
}

impl<'a> ser::SerializeSeq for SeqSerializer<'a> {
    type Ok = ();
    type Error = Never;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<(), Self::Error> {
        todo!()
    }
}

impl<'a> ser::SerializeTuple for SeqSerializer<'a> {
    type Ok = ();
    type Error = Never;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a> ser::SerializeTupleStruct for SeqSerializer<'a> {
    type Ok = ();
    type Error = Never;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a> ser::SerializeTupleVariant for SeqSerializer<'a> {
    type Ok = ();
    type Error = Never;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a> ser::SerializeMap for SeqSerializer<'a> {
    type Ok = ();
    type Error = Never;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a> ser::SerializeStruct for SeqSerializer<'a> {
    type Ok = ();
    type Error = Never;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a> ser::SerializeStructVariant for SeqSerializer<'a> {
    type Ok = ();
    type Error = Never;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

#[inline]
fn int_to_string<I: itoa::Integer>(s: &mut String, i: I) {
    use itoa::Buffer;
    let mut buf = Buffer::new();
    s.push_str(buf.format(i));
}

#[inline]
fn float_to_string<F: ryu::Float>(s: &mut String, f: F) {
    use ryu::Buffer;
    let mut buf = Buffer::new();
    s.push_str(buf.format(f))
}

pub fn to_string<T>(value: &T) -> String
where
    T: ?Sized + Serialize,
{
    let mut ser = Serializer {
        buf: String::with_capacity(16),
    };
    value.serialize(&mut ser).unwrap();
    ser.buf
}
