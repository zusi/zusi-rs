#![cfg(feature = "async")]

use core::mem;

use async_std::io::Read;
use async_std::prelude::*;
use async_trait::async_trait;

use crate::Result;

#[async_trait]
pub trait DeserializeAsync: Sized + Default {
    async fn deserialize<R>(reader: &mut R, length: u32) -> Result<Self>
    where
        R: Read + Unpin + Send;

    async fn deserialize_in_place<R>(reader: &mut R, length: u32, place: &mut Self) -> Result<()>
    where
        R: Read + Unpin + Send,
    {
        *place = DeserializeAsync::deserialize(reader, length).await?;

        Ok(())
    }
}

macro_rules! impl_deserialize_async_for_num {
    ($type:ty) => {
        #[async_trait]
        impl DeserializeAsync for $type {
            async fn deserialize<R>(reader: &mut R, length: u32) -> Result<Self>
            where
                R: Read + Unpin + Send,
            {
                const SIZE: usize = mem::size_of::<$type>();

                if SIZE != length as usize {
                    // todo add error type
                    panic!();
                }

                let mut bts: [u8; SIZE] = [0; SIZE];
                reader.read_exact(&mut bts).await?;
                let result = Self::from_le_bytes(bts);

                Ok(result)
            }
        }
    };
}

impl_deserialize_async_for_num!(u8);
impl_deserialize_async_for_num!(i8);
impl_deserialize_async_for_num!(u16);
impl_deserialize_async_for_num!(i16);
impl_deserialize_async_for_num!(u32);
impl_deserialize_async_for_num!(i32);
impl_deserialize_async_for_num!(u64);
impl_deserialize_async_for_num!(i64);
impl_deserialize_async_for_num!(f32);
impl_deserialize_async_for_num!(f64);

#[async_trait]
impl DeserializeAsync for String {
    async fn deserialize<R>(reader: &mut R, length: u32) -> Result<Self>
    where
        R: Read + Unpin + Send,
    {
        let mut bts = vec![0; length as usize];
        reader.read_exact(&mut bts).await?;

        Ok(String::from_utf8(bts).unwrap())
    }
}

#[async_trait]
impl<T> DeserializeAsync for Option<T>
where
    T: DeserializeAsync,
{
    async fn deserialize<R>(reader: &mut R, length: u32) -> Result<Self>
    where
        R: Read + Unpin + Send,
    {
        Ok(Some(DeserializeAsync::deserialize(reader, length).await?))
    }
}

#[async_trait]
impl<T> DeserializeAsync for Vec<T>
where
    T: DeserializeAsync + Send,
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
    async fn deserialize<R>(reader: &mut R, length: u32) -> Result<Self>
    where
        R: Read + Unpin + Send,
    {
        let vec: Vec<T> = vec![DeserializeAsync::deserialize(reader, length).await?];

        Ok(vec)
    }

    /// Deserializes an element of `Vec<T>` and appends it to the current object.
    async fn deserialize_in_place<R>(reader: &mut R, length: u32, place: &mut Self) -> Result<()>
    where
        R: Read + Unpin + Send,
    {
        place.push(DeserializeAsync::deserialize(reader, length).await?);

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
pub async fn read_header<R>(reader: &mut R) -> Result<Header>
where
    R: Read + Unpin + Send,
{
    let mut buf = [0; 4];
    reader.read_exact(&mut buf).await?;

    let mut len = u32::from_le_bytes(buf);
    if len == 0xFFFFFFFF {
        return Ok(Header::StructEnd);
    } else if len > 2 {
        len -= 2;
    }

    let mut buf = [0; 2];
    reader.read_exact(&mut buf).await?;
    let id = u16::from_le_bytes(buf);

    Ok(Header::Field { id, len })
}

#[cfg(test)]
mod tests {
    use crate::de_async::{read_header, DeserializeAsync, Header};

    #[async_std::test]
    async fn test_read_header() {
        let bts: Vec<u8> = vec![10, 0, 0, 0, 5, 0];

        let result = read_header(&mut &bts[..]).await.unwrap();

        if let Header::Field { id, len } = result {
            assert_eq!(id, 5);
            // we subtract 2 from 10, as id is counted towards the size
            assert_eq!(len, 8);
        } else {
            panic!("Header should be of type Attribute")
        }
    }

    #[async_std::test]
    async fn u8() {
        let bts: Vec<u8> = vec![0x05];

        let result: u8 = u8::deserialize(&mut &bts[..], 1).await.unwrap();

        assert_eq!(result, 5)
    }

    #[async_std::test]
    async fn u16() {
        let bts: Vec<u8> = vec![0x05, 0x01];

        let result: u16 = u16::deserialize(&mut &bts[..], 2).await.unwrap();

        assert_eq!(result, 0x0105)
    }
}
