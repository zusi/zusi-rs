use std::{io::Cursor, marker::PhantomData};

use bytes::{Buf, BufMut, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

use zusi_protocol::parser;
use zusi_protocol::{ClientType, ProtocolError};

use crate::Message;

#[derive(Default)]
pub struct ZusiProtocolCodec<T>
where
    T: ClientType + Send,
{
    phantom: PhantomData<T>,
}

impl<T> Decoder for ZusiProtocolCodec<T>
where
    T: ClientType + Send,
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
    T: ClientType + Send,
{
    type Error = ProtocolError;

    fn encode(&mut self, item: Message<T>, dst: &mut BytesMut) -> Result<(), Self::Error> {
        item.write(&mut dst.writer())?;

        Ok(())
    }
}
