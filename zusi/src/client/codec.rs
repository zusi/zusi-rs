use std::{io::Cursor, marker::PhantomData};

use bytes::{Buf, BufMut, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

use zusi_protocol::parser;
use zusi_protocol::{ClientType, ProtocolError};

use crate::Message;

#[derive(Default)]
pub struct ZusiProtocolCodec<T>
where
    T: ClientType,
{
    phantom: PhantomData<*const T>,
}

impl<T> ZusiProtocolCodec<T>
where
    T: ClientType,
{
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T> Decoder for ZusiProtocolCodec<T>
where
    T: ClientType,
{
    type Item = Message<T>;

    type Error = ProtocolError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match parser::read(src) {
            Ok((input, _node)) => {
                let len = src.len() - input.len();
                let data = src[..len].to_vec();
                let mut data = Cursor::new(data);
                let msg = Message::<T>::receive(&mut data)?;

                src.advance(len);

                Ok(Some(msg))
            }
            Err(nom::Err::Incomplete(_)) => Ok(None),
            Err(_e) => Err(ProtocolError::Deserialization("".into())),
        }
    }
}

impl<T> Encoder<Message<T>> for ZusiProtocolCodec<T>
where
    T: ClientType,
{
    type Error = ProtocolError;

    fn encode(&mut self, item: Message<T>, dst: &mut BytesMut) -> Result<(), Self::Error> {
        // can be optimized after following PR is merged
        // https://github.com/tokio-rs/bytes/pull/478
        let mut data = Vec::new();
        item.write(&mut data)?;
        dst.put(&*data);

        Ok(())
    }
}
