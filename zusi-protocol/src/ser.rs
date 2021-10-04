use core::mem;
use std::io::Write;

use crate::Result;

macro_rules! impl_serialize_for_num {
    ($type:ty) => {
        impl Serialize for $type {
            fn serialize<W>(&self, writer: &mut W, id: u16) -> Result<()>
            where
                W: Write,
            {
                const LEN: u32 = mem::size_of::<$type>() as u32 + 2;

                // according to benchmark its 3x faster to call writer.write_all three times, than
                // to first copy everything into a slice or array and then write it all at once.
                writer.write_all(&LEN.to_le_bytes())?;
                writer.write_all(&id.to_le_bytes())?;
                writer.write_all(&self.to_le_bytes())?;

                Ok(())
            }
        }
    };
}

impl_serialize_for_num!(u8);
impl_serialize_for_num!(i8);
impl_serialize_for_num!(u16);
impl_serialize_for_num!(i16);
impl_serialize_for_num!(u32);
impl_serialize_for_num!(i32);
impl_serialize_for_num!(u64);
impl_serialize_for_num!(i64);
impl_serialize_for_num!(f32);
impl_serialize_for_num!(f64);

/// Serializes a Node or Attribute to an io::Writer.
pub trait Serialize {
    fn serialize<W>(&self, writer: &mut W, id: u16) -> Result<()>
    where
        W: Write;
}

impl<T> Serialize for Option<T>
where
    T: Serialize,
{
    fn serialize<W>(&self, writer: &mut W, id: u16) -> Result<()>
    where
        W: Write,
    {
        if let Some(s) = &self {
            s.serialize(writer, id)?;
        }

        Ok(())
    }
}

impl<T> Serialize for Vec<T>
where
    T: Serialize,
{
    fn serialize<W>(&self, writer: &mut W, id: u16) -> Result<()>
    where
        W: Write,
    {
        for elem in self {
            elem.serialize(writer, id)?;
        }

        Ok(())
    }
}

impl Serialize for String {
    fn serialize<W>(&self, writer: &mut W, id: u16) -> Result<()>
    where
        W: Write,
    {
        self.as_str().serialize(writer, id)
    }
}

impl Serialize for &str {
    fn serialize<W>(&self, writer: &mut W, id: u16) -> Result<()>
    where
        W: Write,
    {
        let len = self.len() as u32 + 2;

        writer.write_all(&len.to_le_bytes())?;
        writer.write_all(&id.to_le_bytes())?;
        writer.write_all(self.as_bytes())?;

        Ok(())
    }
}

pub fn write_node_header<W>(writer: &mut W, id: u16) -> Result<()>
where
    W: Write,
{
    writer.write_all(&crate::NODE_START)?;
    writer.write_all(&id.to_le_bytes())?;

    Ok(())
}

pub fn write_node_end<W>(writer: &mut W) -> Result<()>
where
    W: Write,
{
    writer.write_all(&crate::NODE_END)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::to_bytes;

    // use crate::TestMessage;

    #[test]
    fn u8() {
        let input: u16 = 5;

        let expected: Vec<u8> = vec![0x04, 0x00, 0x00, 0x00, 0x01, 0x00, 0x05, 0x00];

        assert_eq!(to_bytes(&input).unwrap(), expected);
    }

    #[test]
    fn f32() {
        let input: f32 = 5.3;

        let expected: Vec<u8> = vec![0x06, 0x00, 0x00, 0x00, 0x01, 0x00, 0x9a, 0x99, 0xa9, 0x40];

        assert_eq!(to_bytes(&input).unwrap(), expected);
    }

    // #[test]
    // fn test_message() {
    //     let input = TestMessage { field: 1 };
    //
    //     let expected: Vec<u8> = vec![
    //         0, 0, 0, 0, 2, 0, 0x04, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0xFF, 0xFF, 0xFF,
    //         0xFF,
    //     ];
    //
    //     assert_eq!(to_bytes(&input).unwrap(), expected);
    // }

    #[test]
    fn test_string() {
        let input = "Bla".to_string();

        let expected: Vec<u8> = vec![5, 0, 0, 0, 1, 0, 66, 108, 97];

        assert_eq!(to_bytes(&input).unwrap(), expected);
    }

    #[test]
    fn test_str() {
        let input = "Bla";

        let expected: Vec<u8> = vec![5, 0, 0, 0, 1, 0, 66, 108, 97];

        assert_eq!(to_bytes(&input).unwrap(), expected);
    }
}
