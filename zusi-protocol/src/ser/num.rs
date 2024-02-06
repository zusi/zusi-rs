use core::mem;
use std::io::Write;

use crate::{Result, Serialize};

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

#[cfg(test)]
mod tests {
    use crate::to_bytes;

    #[test]
    fn serialize_u8() {
        let input: u16 = 5;

        let expected: Vec<u8> = vec![0x04, 0x00, 0x00, 0x00, 0x01, 0x00, 0x05, 0x00];

        assert_eq!(to_bytes(&input).unwrap(), expected);
    }

    #[test]
    fn serialize_f32() {
        let input: f32 = 5.3;

        let expected: Vec<u8> = vec![0x06, 0x00, 0x00, 0x00, 0x01, 0x00, 0x9a, 0x99, 0xa9, 0x40];

        assert_eq!(to_bytes(&input).unwrap(), expected);
    }
}
