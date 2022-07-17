use std::{io::Cursor, marker::PhantomData};

use bytes::{Buf, BytesMut};
use tokio_util::codec::Decoder;
use zusi_protocol::{ProtocolError, RootMessage};

mod dto;

pub struct ZusiProtocolDecoder<T>
where
    T: RootMessage,
{
    phantom: PhantomData<*const T>,
}

impl<T: RootMessage> ZusiProtocolDecoder<T> {
    pub fn new() -> Self {
        Self { phantom: PhantomData }
    }

    pub fn read_node(&self, src: &mut BytesMut) -> Option<usize> {
        todo!();

        None
    }

    pub fn read_attr(&self, src: &mut BytesMut) -> Option<usize> {
        todo!();

        None
    }
}

impl<T: RootMessage> Decoder for ZusiProtocolDecoder<T> {
    type Item = T;

    type Error = ProtocolError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let len = match self.read_node(src) {
            Some(l) => l,
            None => return Ok(None),
        };

        let data = src[0..len].to_vec();
        let mut data = Cursor::new(data);
        src.advance(len);

        let msg = T::deserialize(&mut data, 0)?;

        Ok(Some(msg))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
