use bytes::BytesMut;
use tokio_util::codec::{Decoder, Encoder};
use zusi::{fahrpult::Fahrpult, verbindungsaufbau::*, Message};
use zusi_protocol::{ProtocolError, RootMessage, ZusiProtocolCodec};

pub type FahrpultMessage = Message<Fahrpult, 2>;

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

fn beispiel_1_msg() -> Result<Option<Message<Fahrpult, 2>>, ProtocolError> {
    Ok(Some(Message {
        verbindungsaufbau: Some(Verbindungsaufbau {
            hello: Some(Hello {
                protokoll_version: 2,
                client_typ: 2,
                name: "Fahrpult".to_string(),
                version: "2.0".to_string(),
            }),
            ack_hello: None,
        }),
        message: None,
    }))
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
    let mut codec = ZusiProtocolCodec::<FahrpultMessage>::new();
    let mut bytes = BytesMut::from(BEISPIEL_1_BYTES);

    let result = consume(&mut codec, &mut bytes);

    assert_eq!(bytes.len(), 0usize);
    assert_eq!(result[0].as_ref().unwrap().as_ref().unwrap(), &beispiel_1_msg().unwrap().unwrap());
}

#[test]
fn test_encoder() {
    let mut codec = ZusiProtocolCodec::<FahrpultMessage>::new();
    let mut output = BytesMut::new();
    let message = FahrpultMessage::default();

    codec.encode(message, &mut output).unwrap();

    assert_eq!(output.len(), 0);
}
