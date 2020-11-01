use std::convert::TryInto;
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

    #[no_mangle]
    fn deserialize_struct<R>(reader: &mut R) -> Result<Self>
        where
        R: Read, {
        let _header = read_header(reader);

        Deserialize::deserialize(reader, 0)
    }
}

macro_rules! impl_deserialize_for_num {
    ($type:ty) => {
        impl Deserialize for $type {
            fn deserialize<R>(reader: &mut R, length: u32) -> Result<Self>
            where
                R: Read,
            {
                let mut bts = vec![0; length as usize];
                reader.read_exact(&mut bts)?;

                let result = Self::from_le_bytes(bts.try_into().unwrap());

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

    let mut len = u32::from_le_bytes(buf.try_into().unwrap());
    if len == 0xFFFFFFFF {
        return Ok(Header::StructEnd);
    } else if len > 2 {
        len -= 2;
    }

    reader.read_exact(&mut buf[0..2])?;
    let id = u16::from_le_bytes(buf[..2].try_into().unwrap());

    Ok(Header::Field { id, len })
}

#[cfg(test)]
mod tests {
    use crate::de::{read_header, Deserialize, Header};

    #[test]
    fn test_read_header() {
        let mut bts: Vec<u8> = vec![10, 0, 0, 0, 5, 0];

        let result = read_header(&mut &bts[..]).unwrap();

        if let Header::Field { id, len } = result {
            assert_eq!(id, 5);
            assert_eq!(len, 10);
        } else {
            panic!("Header should be of type Attribute")
        }
    }

    #[test]
    fn u8() {
        let mut bts: Vec<u8> = vec![0x05];

        let result: u8 = u8::deserialize(&mut &bts[..]).unwrap();

        assert_eq!(result, 5)
    }
}
