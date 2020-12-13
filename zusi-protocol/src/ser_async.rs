#![cfg(feature = "async")]

use core::mem;

use async_std::io::Write;
use async_std::prelude::*;
use async_trait::async_trait;

use crate::Result;

macro_rules! impl_serialize_async_for_num {
    ($type:ty) => {
        #[async_trait]
        impl SerializeAsync for $type {
            async fn serialize<W>(&self, writer: &mut W, id: u16) -> Result<()>
            where
                W: Write + Unpin + Send,
            {
                const LEN: u32 = mem::size_of::<$type>() as u32 + 2;

                // according to benchmark its 3x faster to call writer.write_all three times, than
                // to first copy everything into a slice or array and then write it all at once.
                writer.write_all(&LEN.to_le_bytes()).await?;
                writer.write_all(&id.to_le_bytes()).await?;
                writer.write_all(&self.to_le_bytes()).await?;

                Ok(())
            }
        }
    };
}

impl_serialize_async_for_num!(u8);
impl_serialize_async_for_num!(i8);
impl_serialize_async_for_num!(u16);
impl_serialize_async_for_num!(i16);
impl_serialize_async_for_num!(u32);
impl_serialize_async_for_num!(i32);
impl_serialize_async_for_num!(u64);
impl_serialize_async_for_num!(i64);
impl_serialize_async_for_num!(f32);
impl_serialize_async_for_num!(f64);

#[async_trait]
/// Serializes a Node or Attribute to an io::Writer.
pub trait SerializeAsync {
    async fn serialize<W>(&self, writer: &mut W, id: u16) -> Result<()>
    where
        W: Write + Unpin + Send;
}

#[async_trait]
impl<T> SerializeAsync for Option<T>
where
    T: SerializeAsync + Sync,
{
    async fn serialize<W>(&self, writer: &mut W, id: u16) -> Result<()>
    where
        W: Write + Unpin + Send,
    {
        if let Some(s) = &self {
            s.serialize(writer, id).await?;
        }

        Ok(())
    }
}

#[async_trait]
impl<T> SerializeAsync for Vec<T>
where
    T: SerializeAsync + Sync,
{
    async fn serialize<W>(&self, writer: &mut W, id: u16) -> Result<()>
    where
        W: Write + Unpin + Send,
    {
        for elem in self {
            elem.serialize(writer, id).await?;
        }

        Ok(())
    }
}

#[async_trait]
impl SerializeAsync for String {
    async fn serialize<W>(&self, writer: &mut W, id: u16) -> Result<()>
    where
        W: Write + Unpin + Send,
    {
        self.as_str().serialize(writer, id).await
    }
}

#[async_trait]
impl SerializeAsync for &str {
    async fn serialize<W>(&self, writer: &mut W, id: u16) -> Result<()>
    where
        W: Write + Unpin + Send,
    {
        let len = self.len() as u32 + 2;

        writer.write_all(&len.to_le_bytes()).await?;
        writer.write_all(&id.to_le_bytes()).await?;
        writer.write_all(&self.as_bytes()).await?;

        Ok(())
    }
}

pub async fn write_node_header<W>(writer: &mut W, id: u16) -> Result<()>
where
    W: Write + Unpin + Send,
{
    writer.write_all(&crate::NODE_START).await?;
    writer.write_all(&id.to_le_bytes()).await?;

    Ok(())
}

pub async fn write_node_end<W>(writer: &mut W) -> Result<()>
where
    W: Write + Unpin + Send,
{
    writer.write_all(&crate::NODE_END).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use async_std::io;

    use crate::ser_async::SerializeAsync;

    async fn to_bytes<T>(value: &T) -> Result<Vec<u8>, io::Error>
    where
        T: SerializeAsync,
    {
        let mut c = Vec::new();

        value.serialize(&mut c, 0x01).await?;

        Ok(c)
    }

    #[async_std::test]
    async fn u8() {
        let input: u16 = 5;

        let expected: Vec<u8> = vec![0x04, 0x00, 0x00, 0x00, 0x01, 0x00, 0x05, 0x00];

        assert_eq!(to_bytes(&input).await.unwrap(), expected);
    }

    #[async_std::test]
    async fn f32() {
        let input: f32 = 5.3;

        let expected: Vec<u8> = vec![0x06, 0x00, 0x00, 0x00, 0x01, 0x00, 0x9a, 0x99, 0xa9, 0x40];

        assert_eq!(to_bytes(&input).await.unwrap(), expected);
    }

    #[async_std::test]
    async fn test_string() {
        let input = "Bla".to_string();

        let expected: Vec<u8> = vec![5, 0, 0, 0, 1, 0, 66, 108, 97];

        assert_eq!(to_bytes(&input).await.unwrap(), expected);
    }

    #[async_std::test]
    async fn test_str() {
        let input = "Bla";

        let expected: Vec<u8> = vec![5, 0, 0, 0, 1, 0, 66, 108, 97];

        assert_eq!(to_bytes(&input).await.unwrap(), expected);
    }
}
