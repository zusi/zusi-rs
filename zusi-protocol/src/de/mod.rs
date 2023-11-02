mod num;

use core::mem;
use std::io::Read;

use log::warn;

use crate::{ProtocolError, Result};

pub trait Deserialize: Sized {
    fn deserialize<R>(reader: &mut R, length: u32) -> Result<Self>
    where
        R: Read;

    fn deserialize_in_place<R>(reader: &mut R, length: u32, place: &mut Self) -> Result<()>
    where
        R: Read,
    {
        *place = Deserialize::deserialize(reader, length)?;

        Ok(())
    }
}

impl Deserialize for String {
    fn deserialize<R>(reader: &mut R, length: u32) -> Result<Self>
    where
        R: Read,
    {
        let mut bts = vec![0; length as usize];
        reader.read_exact(&mut bts)?;

        Ok(String::from_utf8(bts).unwrap())
    }
}

impl<T> Deserialize for Option<T>
where
    T: Deserialize,
{
    fn deserialize<R>(reader: &mut R, length: u32) -> Result<Self>
    where
        R: Read,
    {
        Ok(Some(Deserialize::deserialize(reader, length)?))
    }
}

impl<T> Deserialize for Vec<T>
where
    T: Deserialize,
{
    /// # Warning
    /// This function is a bit funky to use for a Vec.
    /// By the nature of this TCP protocol, we don't know if a read value is the first or a
    /// consecutive following value of the `Vec<T>`.
    ///
    /// The caller of this function has to handle the case of a `Vec<T>` with more than one value
    /// and has to merge the returned `Vec<T>` with a single value with the values deserialized
    /// before.
    ///
    /// When possible `deserialize_in_place` should be used instead, as that function has access
    /// to the underlying object and can append the deserialized element.
    fn deserialize<R>(reader: &mut R, length: u32) -> Result<Self>
    where
        R: Read,
    {
        let vec: Vec<T> = vec![Deserialize::deserialize(reader, length)?];

        Ok(vec)
    }

    /// Deserializes an element of `Vec<T>` and appends it to the current object.
    fn deserialize_in_place<R>(reader: &mut R, length: u32, place: &mut Self) -> Result<()>
    where
        R: Read,
    {
        place.push(Deserialize::deserialize(reader, length)?);

        Ok(())
    }
}

#[derive(PartialEq, Debug)]
pub enum Header {
    StructEnd,
    Field {
        id: u16,
        /// When `len == 0` a struct follows.
        len: u32,
    },
}

/// Reads a Node or Attribute header
///
/// Header is composed of u32 length and u8 id.
///
/// When `len == 0` a struct follows and if `len == 0xFFFFFFFF` we are at the end of a struct.
pub fn read_header<R>(reader: &mut R) -> Result<Header>
where
    R: Read,
{
    let mut buf = [0; 4];
    reader.read_exact(&mut buf)?;

    let mut len = u32::from_le_bytes(buf);
    if len == 0xFFFFFFFF {
        return Ok(Header::StructEnd);
    } else if len > 2 {
        len -= 2;
    }

    let mut buf = [0; 2];
    reader.read_exact(&mut buf)?;
    let id = u16::from_le_bytes(buf);

    Ok(Header::Field { id, len })
}

pub fn read_unknown_field<R>(reader: &mut R, header: Header) -> Result<()>
where
    R: Read,
{
    if let Header::Field { id, len } = header {
        warn!("reading unknown field {} with len={}", id, len);

        if len == 0 {
            // we found a unknown struct and go recursive into it

            while let Header::Field { id, len } = read_header(reader)? {
                read_unknown_field(reader, Header::Field { id, len })?
            }
        } else {
            let mut buf = vec![0; len as usize];
            reader.read_exact(&mut buf)?;
        }
    } else {
        return Err(ProtocolError::Deserialization(
            "calling read_unknown_field is not supported nor useful".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::de::{read_header, Deserialize, Header};

    #[test]
    fn test_read_header() {
        let bts: Vec<u8> = vec![10, 0, 0, 0, 5, 0];

        let result = read_header(&mut &bts[..]).unwrap();

        if let Header::Field { id, len } = result {
            assert_eq!(id, 5);
            // we subtract 2 from 10, as id is counted towards the size
            assert_eq!(len, 8);
        } else {
            panic!("Header should be of type Attribute")
        }
    }

    #[test]
    fn u8() {
        let bts: Vec<u8> = vec![0x05];

        let result: u8 = u8::deserialize(&mut &bts[..], 1).unwrap();

        assert_eq!(result, 5)
    }

    #[test]
    fn u8_wrong_length() {
        let bts: Vec<u8> = vec![0x05];

        let _result = u8::deserialize(&mut &bts[..], 2)
            .expect_err("memory size (1) of type u8 differs from reported length 2");
    }
}
