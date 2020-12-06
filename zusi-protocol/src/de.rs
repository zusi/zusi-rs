use core::mem;
use std::io::Read;

use crate::Result;

pub trait Deserialize: Sized + Default {
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

macro_rules! impl_deserialize_for_num {
    ($type:ty) => {
        impl Deserialize for $type {
            fn deserialize<R>(reader: &mut R, length: u32) -> Result<Self>
            where
                R: Read,
            {
                const SIZE: usize = mem::size_of::<$type>();

                if SIZE != length as usize {
                    // todo add error type
                    panic!();
                }

                let mut bts: [u8; SIZE] = [0; SIZE];
                reader.read_exact(&mut bts)?;
                let result = Self::from_le_bytes(bts);

                Ok(result)
            }
        }
    };
}

impl_deserialize_for_num!(u8);
impl_deserialize_for_num!(i8);
impl_deserialize_for_num!(u16);
impl_deserialize_for_num!(i16);
impl_deserialize_for_num!(u32);
impl_deserialize_for_num!(i32);
impl_deserialize_for_num!(u64);
impl_deserialize_for_num!(i64);
impl_deserialize_for_num!(f32);
impl_deserialize_for_num!(f64);

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
    Field { id: u16, len: u32 },
}

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
}
