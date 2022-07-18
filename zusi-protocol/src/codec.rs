use std::{io::Cursor, marker::PhantomData};

use crate::{parser, ProtocolError, RootMessage};
use bytes::{Buf, BufMut, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

#[derive(Default)]
pub struct ZusiProtocolCodec<T>
where
    T: RootMessage,
{
    phantom: PhantomData<*const T>,
}

impl<T: RootMessage> ZusiProtocolCodec<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T: RootMessage> Decoder for ZusiProtocolCodec<T> {
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

impl<T: RootMessage> Encoder<T> for ZusiProtocolCodec<T> {
    type Error = ProtocolError;

    fn encode(&mut self, item: T, dst: &mut BytesMut) -> Result<(), Self::Error> {
        // can be optimized after following PR is merged
        // https://github.com/tokio-rs/bytes/pull/478
        let mut data = Vec::new();
        item.serialize(&mut data, 0)?;
        dst.put(&*data);

        Ok(())
    }
}
