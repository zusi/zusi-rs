use std::convert::TryInto;
use std::io::Read;

use crate::Result;

// pub trait Deserialize: Sized {
//     fn deserialize(deserializer: Deserializer, len: u32) -> Result<Self>;
//
//     fn deserialize_in_place(deserializer: Deserializer, len: u32, place: &mut Self) -> Result<()> {
//         *place = Deserialize::deserialize(deserializer, len)?;
//
//         Ok(())
//     }
// }

pub trait Deserialize: Sized {
    fn deserialize<R>(reader: &mut R) -> Result<()>
    where
        R: Read;
}

impl Deserialize for u8 {
    fn deserialize<R>(_reader: &mut R) -> Result<()>
    where
        R: Read,
    {
        Ok(())
    }
}

pub enum Header {
    StructEnd,
    StructBegin { id: u16 },
    Attribute { id: u16, len: u32 },
}

pub fn read_header<R>(reader: &mut R) -> Result<Header>
where
    R: Read,
{
    let mut buf = [0; 4];
    reader.read_exact(&mut buf)?;

    let len = u32::from_le_bytes(buf.try_into().unwrap());
    if len == 0xFFFFFFFF {
        return Ok(Header::StructEnd);
    }

    reader.read_exact(&mut buf[0..2])?;
    let id = u16::from_le_bytes(buf[..2].try_into().unwrap());

    if len == 0x0 {
        Ok(Header::StructBegin { id })
    } else {
        Ok(Header::Attribute { id, len })
    }
}

// pub enum Field {
//     StructEnd,
//     Field{id: u16, field: Vec<u8>},
// }
//
// pub fn read_field<R>(reader: &mut R) -> Result<Field>
// where
//     R: Read,
// {
//     let header = read_header(reader)?;
//     if header == Header::StructEnd {
//         return Ok(Field::StructEnd);
//     }
//
//     let mut buf = vec![0; len as usize];
//     reader.read_exact(&mut buf)?;
// }

#[cfg(test)]
mod tests {
    use crate::de::{read_header, Header};

    #[test]
    fn test_read_header() {
        let mut bts: Vec<u8> = vec![10, 0, 0, 0, 5, 0];

        let result = read_header(&mut &bts[..]).unwrap();

        if let Header::Attribute { id, len } = result {
            assert_eq!(id, 5);
            assert_eq!(len, 10);
        } else {
            panic!("Header should be of type Attribute")
        }
    }
}
