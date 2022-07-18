use std::{io::Cursor, marker::PhantomData};

use bytes::{Buf, BytesMut};
use tokio_util::codec::Decoder;
use zusi_protocol::{ProtocolError, RootMessage};

mod parser;

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
}

impl<T: RootMessage> Decoder for ZusiProtocolDecoder<T> {
    type Item = T;

    type Error = ProtocolError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match parser::read(src) {
            Ok((input, _node)) => {
                let len = src.len() - input.len();
                let data = src[..len].to_vec();
                let mut data = Cursor::new(data);
                let msg = T::deserialize(&mut data, 0)?;

                src.advance(len);

                Ok(Some(msg))
            }
            Err(nom::Err::Incomplete(_)) => Ok(None),
            Err(_e) => Err(ProtocolError::Deserialization("".into())),
        }
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
