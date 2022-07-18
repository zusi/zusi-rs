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
    use bytes::BytesMut;
    use tokio_util::codec::Decoder;
    use zusi_fahrpult::Message;

    use crate::ZusiProtocolDecoder;

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
        let mut decoder = ZusiProtocolDecoder::<Message>::new();
        let mut bts = BytesMut::from(BEISPIEL_1_BYTES);
        let result = decoder.decode(&mut bts).unwrap();

        assert_ne!(result, None);
    }
}
