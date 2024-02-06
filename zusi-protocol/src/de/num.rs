use crate::{Deserialize, ProtocolError, Result};
use core::mem;
use std::io::Read;

macro_rules! impl_deserialize_for_num {
    ($type:ty) => {
        impl Deserialize for $type {
            fn deserialize<R>(reader: &mut R, length: u32) -> Result<Self>
            where
                R: Read,
            {
                const SIZE: usize = mem::size_of::<$type>();

                if SIZE != length as usize {
                    return Err(ProtocolError::Deserialization(format!(
                        "memory size ({}) of type {} differs from reported length {}",
                        SIZE,
                        stringify!($type),
                        length
                    )));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_u8() {
        let bts: Vec<u8> = vec![0x05];

        let result: u8 = u8::deserialize(&mut &bts[..], 1).unwrap();

        assert_eq!(result, 5)
    }

    #[test]
    fn deserialize_u8_wrong_length() {
        let bts: Vec<u8> = vec![0x05];

        let _result = u8::deserialize(&mut &bts[..], 2)
            .expect_err("memory size (1) of type u8 differs from reported length 2");
    }
}
