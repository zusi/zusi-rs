use std::{io::Cursor, marker::PhantomData};

use bytes::{Buf, BufMut, BytesMut};
use tokio_util::codec::{Decoder, Encoder};
use zusi_protocol::{ProtocolError, RootMessage};

mod parser;

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
        let mut data = Vec::new();
        item.serialize(&mut data, 0)?;
        dst.put(&*data);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;
    use tokio_util::codec::Decoder;
    use zusi_fahrpult::Message;
    use super::*;
    use zusi_protocol::{ProtocolError, RootMessage};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    static BEISPIEL_1_BYTES: &'static [u8] = &[
        0x00, 0x00, 0x00, 0x00, // Länge 0 Bytes → es beginnt ein Knoten
        0x01, 0x00, // ID 1: Verbindungsaufbau
        0x00, 0x00, 0x00, 0x00, // Länge 0 Bytes → es beginnt ein Knoten
        0x01, 0x00, // ID 1: HELLO-Befehl
        0x04, 0x00, 0x00, 0x00, // Länge 4 Bytes → es folgt ein Attribut, Länge 4 bytes
        0x01, 0x00, // ID x0001: Protokoll-Version
        0x02, 0x00, // Protokoll-Version „2“ (Word)
        0x04, 0x00, 0x00, 0x00, // Länge 4 Bytes → es folgt ein Attribut, Länge 4 bytes
        0x02, 0x00, // ID x0002: Client-Typ
        0x02, 0x00, // Client-Typ „Fahrpult“ (Word)
        0x0A, 0x00, 0x00, 0x00, // Länge 10 Bytes → es folgt ein Attribut
        0x03, 0x00, // ID x0003: Klartextstring
        0x46, 0x61, 0x68, 0x72, 0x70, 0x75, 0x6C,
        0x74, // String „Fahrpult“ (8 Zeichen, da 2 bytes für die ID)
        0x05, 0x00, 0x00, 0x00, // Länge 5 Bytes → es folgt ein Attribut
        0x04, 0x00, // ID x0004: Version
        0x32, 0x2E, 0x30, // String „2.0“
        0xFF, 0xFF, 0xFF, 0xFF, // Ende Knoten
        0xFF, 0xFF, 0xFF, 0xFF, // Ende Knoten
    ];

    #[test]
    fn run_example() {
        let mut decoder = ZusiProtocolCodec::<Message>::new();
        let mut bts = BytesMut::from(BEISPIEL_1_BYTES);
        let result = decoder.decode(&mut bts).unwrap();

        assert_ne!(result, None);
    }


    fn consume<T: RootMessage>(
        codec: &mut ZusiProtocolCodec<T>,
        bytes: &mut BytesMut,
    ) -> Vec<Result<Option<T>, ProtocolError>> {
        let mut result = Vec::new();
        loop {
            match codec.decode(bytes) {
                Ok(None) => {
                    break;
                }
                output => result.push(output),
            }
        }
        return result;
    }

    #[test]
    fn test_decoder() {
        let mut codec = ZusiProtocolCodec::<Message>::new();
        let mut bytes = BytesMut::from(BEISPIEL_1_BYTES);

        let result = consume(&mut codec, &mut bytes);

        assert_eq!(bytes.len(), 0usize);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_encoder() {
        let mut codec = ZusiProtocolCodec::<Message>::new();
        let mut output = BytesMut::new();
        let message = Message::default();

        codec.encode(message, &mut output).unwrap();

        assert_eq!(output.len(), 0);
    }
}
