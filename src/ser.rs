use std::fmt::Display;

use serde::ser::{self, Serialize, SerializeStruct, SerializeTuple};

use crate::{Error, Result};

pub struct Serializer {
    // This string starts empty and JSON is appended as values are serialized.
    output: Vec<u8>,
}

pub fn to_bytes<T>(value: &T) -> Result<Vec<u8>>
    where
        T: Serialize,
{
    let mut serializer = Serializer {
        output: Default::default(),
    };
    value.serialize(&mut serializer)?;

    Ok(serializer.output)
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    // Associated types for keeping track of additional state while serializing
    // compound data structures like sequences and maps. In this case no
    // additional state is required beyond what is already stored in the
    // Serializer struct.
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        let mut bytes = v.to_le_bytes().to_vec();

        self.output.append(&mut bytes);

        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        let mut bytes = v.to_le_bytes().to_vec();

        self.output.append(&mut bytes);

        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok>
        where
            T: Serialize,
    {
        unimplemented!()
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_newtype_struct<T: ?Sized>(self, name: &'static str, value: &T) -> Result<Self::Ok>
        where
            T: Serialize,
    {
        unimplemented!()
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
        where
            T: Serialize,
    {
        unimplemented!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        unimplemented!()
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        unimplemented!()
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        unimplemented!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        unimplemented!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        unimplemented!()
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct> {


        unimplemented!()
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        unimplemented!()
    }

    fn collect_str<T: ?Sized>(self, value: &T) -> Result<Self::Ok>
        where
            T: Display,
    {
        unimplemented!()
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
        where
            T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
        where
            T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
        where
            T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
        where
            T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<()>
        where
            T: Serialize,
    {
        unimplemented!()
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<()>
        where
            T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()>
        where
            T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()>
        where
            T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok> {
        unimplemented!()
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use serde_derive::Serialize;

    use super::to_bytes;

    #[test]
    #[rustfmt::skip]
    fn test_struct() {
        #[derive(Serialize)]
        struct Test {
            protokoll_version: u16,
            client_typ: u16,
            name: String,
            version: String,
        }

        let test = Test {
            protokoll_version: 2,
            client_typ: 2,
            name: "Fahrpult".to_string(),
            version: "2.0".to_string(),
        };

        let expected: Vec<u8> = vec![
            0x04, 0x00, 0x00, 0x00, // Länge 4 Bytes → es folgt ein Attribut, Länge 4 bytes
            0x01, 0x00, // ID x0001: Protokoll-Version
            0x02, 0x00, // Protokoll-Version "2" (Word)
            0x04, 0x00, 0x00, 0x00, // Länge 4 Bytes → es folgt ein Attribut, Länge 4 bytes
            0x02, 0x00, // ID x0002: Client-Typ
            0x02, 0x00, // Client-Typ "Fahrpult" (Word)
            0x0A, 0x00, 0x00, 0x00, // Länge 10 Bytes → es folgt ein Attribut
            0x03, 0x00, // ID x0003: Klartextstring
            0x46, 0x61, 0x68, 0x72, 0x70, 0x75, 0x6C, 0x74, // String "Fahrpult" (8 Zeichen, da 2 bytes für die ID)
            0x05, 0x00, 0x00, 0x00, // Länge 5 Bytes → es folgt ein Attribut
            0x04, 0x00, // ID x0004: Version
            0x32, 0x2E, 0x30, // String "2.0"
        ];
        assert_eq!(to_bytes(&test).unwrap(), expected);
    }

    #[test]
    fn test_u16() {
        let test = 5_u16;

        let expected: Vec<u8> = vec![0x05, 0x00];

        assert_eq!(to_bytes(&test).unwrap(), expected);
    }
}
