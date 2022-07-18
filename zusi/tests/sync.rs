use std::io::{Cursor, Read, Write};

use zusi::verbindungsaufbau::*;
use zusi::{
    fahrpult::{self, *},
    Message,
};
use zusi_fahrpult::ZusiClientError;
use zusi_protocol::{Deserialize, Serialize};

// use crate::fahrpult::{AckNeededData, DataFtd, Fahrpult, FuehrerstandsAnzeigen, NeededData};
// use crate::verbindungsaufbau::{AckHello, Hello, Verbindungsaufbau};
// use crate::{receive_message, send_fahrpult, send_verbindungsaufbau, Message};

pub type Result<T> = std::result::Result<T, ZusiClientError>;
pub type FahrpultMessage = Message<Fahrpult, 2>;

fn receive_message<R: Read>(mut reader: &mut R) -> Result<FahrpultMessage> {
    Ok(Message::deserialize(&mut reader, 0)?)
}

pub fn receive_fahrpult<R: Read>(mut reader: &mut R) -> Result<Fahrpult> {
    let msg: Message<Fahrpult, 2> = receive_message(&mut reader)?;

    if let Some(m) = msg.message {
        return Ok(m);
    }

    Err(ZusiClientError::WrongMessageType)
}

pub fn receive_verbindungsaufbau<R: Read>(mut reader: &mut R) -> Result<Verbindungsaufbau> {
    let msg: Message<Fahrpult, 2> = receive_message(&mut reader)?;

    if let Some(m) = msg.verbindungsaufbau {
        return Ok(m);
    }

    Err(ZusiClientError::WrongMessageType)
}

pub fn send_verbindungsaufbau<W>(msg: Verbindungsaufbau, mut writer: &mut W) -> Result<()>
where
    W: Write,
{
    msg.serialize(&mut writer, 1)?;

    Ok(())
}

pub fn send_fahrpult<W>(msg: fahrpult::Fahrpult, mut writer: &mut W) -> Result<()>
where
    W: Write,
{
    msg.serialize(&mut writer, 2)?;

    Ok(())
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

fn beispiel_1_msg() -> Message<Fahrpult, 2> {
    Message {
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
    }
}

static BEISPIEL_2_BYTES: &'static [u8] = &[
    0x00, 0x00, 0x00, 0x00, // Länge 0 Bytes → es beginnt ein Knoten
    0x01, 0x00, // ID 1: Verbindungsaufbau
    0x00, 0x00, 0x00, 0x00, // Länge 0 Bytes → es beginnt ein Knoten
    0x02, 0x00, // ID 2: ACK_HELLO-Befehl
    0x09, 0x00, 0x00, 0x00, // Länge 9 Bytes → es folgt ein Attribut
    0x01, 0x00, // ID x0001: Zusi-Version
    0x33, 0x2E, 0x30, 0x2E, 0x31, 0x2E, 0x30, // String „3.0.1.0“
    0x03, 0x00, 0x00, 0x00, // Länge 3 Bytes → es folgt ein Attribut
    0x02, 0x00, // ID x0002: Zusi-Verbindungsinfo
    0x30, // String „0“
    0x03, 0x00, 0x00, 0x00, // Länge 3 Bytes → es folgt ein Attribut
    0x03, 0x00, // ID x0003: Ergebnis
    0x00, // 0 (Byte) -> Verbindung akzeptiert
    0x0A, 0x00, 0x00, 0x00, // Länge 10 Bytes → es folgt ein Attribut
    0x04, 0x00, // ID x0004: Fahrplananfangszeit
    0x00, 0x00, 0x00, 0x00, 0xD0, 0x35, 0xE4,
    0x40, // 41390,5 (double) -> Anfangszeit in Tagen
    0xFF, 0xFF, 0xFF, 0xFF, // Ende Knoten
    0xFF, 0xFF, 0xFF, 0xFF, // Ende Knoten
];

fn beispiel_2_msg() -> Message<Fahrpult, 2> {
    Message {
        verbindungsaufbau: Some(Verbindungsaufbau {
            hello: None,
            ack_hello: Some(AckHello {
                zusi_version: "3.0.1.0".to_string(),
                zusi_verbindungsinfo: "0".to_string(),
                error_code: 0,
                fahrplan_start_zeit: Some(41390.5),
                protokoll_version: None,
            }),
        }),
        message: None,
    }
}

static BEISPIEL_3_BYTES: &'static [u8] = &[
    0x00, 0x00, 0x00, 0x00, // Länge 0 Bytes → es beginnt ein Knoten
    0x02, 0x00, // ID 0x0002: Client-Anwendung Typ 2 (Fahrpult)
    0x00, 0x00, 0x00, 0x00, // Länge 0 Bytes → es beginnt ein Knoten
    0x03, 0x00, // ID 0x0003: NEEDED_DATA-Befehl
    0x00, 0x00, 0x00, 0x00, // Länge 0 Bytes → es beginnt ein Knoten
    0x0A, 0x00, // ID x000A: Führerstandsanzeigen
    0x04, 0x00, 0x00, 0x00, // Länge 4 Bytes → es folgt ein Attribut
    0x01, 0x00, // ID 0x0001: Führerstands ID
    0x01, 0x00, // Nr. 1: Geschwindigkeit (Word)
    0x04, 0x00, 0x00, 0x00, // Länge 4 Bytes → es folgt ein Attribut
    0x01, 0x00, // ID 0x0001: Führerstands ID
    0x1B, 0x00, // Nr. 1B: Schleudern (Word)
    0xFF, 0xFF, 0xFF, 0xFF, // Ende Knoten
    0xFF, 0xFF, 0xFF, 0xFF, // Ende Knoten
    0xFF, 0xFF, 0xFF, 0xFF, // Ende Knoten
];

static BEISPIEL_3_BYTES_WITH_UNKNOWN_ATTRIBUTE: &'static [u8] = &[
    0x00, 0x00, 0x00, 0x00, // Länge 0 Bytes → es beginnt ein Knoten
    0x02, 0x00, // ID 0x0002: Client-Anwendung Typ 2 (Fahrpult)
    0x00, 0x00, 0x00, 0x00, // Länge 0 Bytes → es beginnt ein Knoten
    0x03, 0x00, // ID 0x0003: NEEDED_DATA-Befehl
    0x00, 0x00, 0x00, 0x00, // Länge 0 Bytes → es beginnt ein Knoten
    0x0A, 0x00, // ID x000A: Führerstandsanzeigen
    0x04, 0x00, 0x00, 0x00, // Länge 4 Bytes → es folgt ein Attribut
    0x01, 0x00, // ID 0x0001: Führerstands ID
    0x01, 0x00, // Nr. 1: Geschwindigkeit (Word)
    0x04, 0x00, 0x00, 0x00, // Länge 4 Bytes → es folgt ein Attribut
    0x01, 0x0F, // ID 0x0F01: Unbekanntes Feld
    0x34, 0x12, //
    0x04, 0x00, 0x00, 0x00, // Länge 4 Bytes → es folgt ein Attribut
    0x01, 0x00, // ID 0x0001: Führerstands ID
    0x1B, 0x00, // Nr. 1B: Schleudern (Word)
    0xFF, 0xFF, 0xFF, 0xFF, // Ende Knoten
    0xFF, 0xFF, 0xFF, 0xFF, // Ende Knoten
    0xFF, 0xFF, 0xFF, 0xFF, // Ende Knoten
];

static BEISPIEL_3_BYTES_WITH_UNKNOWN_NODE: &'static [u8] = &[
    0x00, 0x00, 0x00, 0x00, // Länge 0 Bytes → es beginnt ein Knoten
    0x02, 0x00, // ID 0x0002: Client-Anwendung Typ 2 (Fahrpult)
    0x00, 0x00, 0x00, 0x00, // Länge 0 Bytes → es beginnt ein Knoten
    0x03, 0x00, // ID 0x0003: NEEDED_DATA-Befehl
    0x00, 0x00, 0x00, 0x00, // Länge 0 Bytes → es beginnt ein Knoten
    0x0A, 0x00, // ID x000A: Führerstandsanzeigen
    0x04, 0x00, 0x00, 0x00, // Länge 4 Bytes → es folgt ein Attribut
    0x01, 0x00, // ID 0x0001: Führerstands ID
    0x01, 0x00, // Nr. 1: Geschwindigkeit (Word)
    0x00, 0x00, 0x00, 0x00, // Länge 0 Bytes → es beginnt ein Knoten
    0x0F, 0x0F, // ID 0x0F0F: Unbekannter Knoten
    0x04, 0x00, 0x00, 0x00, // Länge 4 Bytes → es folgt ein Attribut
    0x01, 0x0F, // ID 0x0F01: Unbekanntes Feld 1
    0x34, 0x12, //
    0x04, 0x00, 0x00, 0x00, // Länge 4 Bytes → es folgt ein Attribut
    0x01, 0x0A, // ID 0x0F01: Unbekanntes Feld 2
    0x34, 0x12, //
    0xFF, 0xFF, 0xFF, 0xFF, // Ende Knoten
    0x04, 0x00, 0x00, 0x00, // Länge 4 Bytes → es folgt ein Attribut
    0x01, 0x00, // ID 0x0001: Führerstands ID
    0x1B, 0x00, // Nr. 1B: Schleudern (Word)
    0xFF, 0xFF, 0xFF, 0xFF, // Ende Knoten
    0xFF, 0xFF, 0xFF, 0xFF, // Ende Knoten
    0xFF, 0xFF, 0xFF, 0xFF, // Ende Knoten
];

static BEISPIEL_3_BYTES_WITH_UNKNOWN_NODE_NESTED: &'static [u8] = &[
    0x00, 0x00, 0x00, 0x00, // Länge 0 Bytes → es beginnt ein Knoten
    0x02, 0x00, // ID 0x0002: Client-Anwendung Typ 2 (Fahrpult)
    0x00, 0x00, 0x00, 0x00, // Länge 0 Bytes → es beginnt ein Knoten
    0x03, 0x00, // ID 0x0003: NEEDED_DATA-Befehl
    0x00, 0x00, 0x00, 0x00, // Länge 0 Bytes → es beginnt ein Knoten
    0x0A, 0x00, // ID x000A: Führerstandsanzeigen
    0x04, 0x00, 0x00, 0x00, // Länge 4 Bytes → es folgt ein Attribut
    0x01, 0x00, // ID 0x0001: Führerstands ID
    0x01, 0x00, // Nr. 1: Geschwindigkeit (Word)
    0x00, 0x00, 0x00, 0x00, // Länge 0 Bytes → es beginnt ein Knoten
    0x0F, 0x0F, // ID 0x0F0F: Unbekannter Knoten
    0x04, 0x00, 0x00, 0x00, // Länge 4 Bytes → es folgt ein Attribut
    0x01, 0x0F, // ID 0x0F01: Unbekanntes Feld 1
    0x34, 0x12, //
    0x04, 0x00, 0x00, 0x00, // Länge 4 Bytes → es folgt ein Attribut
    0x01, 0x0A, // ID 0x0F01: Unbekanntes Feld 2
    0x34, 0x12, //
    0x00, 0x00, 0x00, 0x00, // Länge 0 Bytes → es beginnt ein Knoten
    0x0F, 0x0A, // ID 0x0F0F: Unbekannter Knoten
    0x04, 0x00, 0x00, 0x00, // Länge 4 Bytes → es folgt ein Attribut
    0x01, 0x0F, // ID 0x0F01: Unbekanntes Feld 1
    0x34, 0x12, //
    0xFF, 0xFF, 0xFF, 0xFF, // Ende Knoten
    0xFF, 0xFF, 0xFF, 0xFF, // Ende Knoten
    0x04, 0x00, 0x00, 0x00, // Länge 4 Bytes → es folgt ein Attribut
    0x01, 0x00, // ID 0x0001: Führerstands ID
    0x1B, 0x00, // Nr. 1B: Schleudern (Word)
    0xFF, 0xFF, 0xFF, 0xFF, // Ende Knoten
    0xFF, 0xFF, 0xFF, 0xFF, // Ende Knoten
    0xFF, 0xFF, 0xFF, 0xFF, // Ende Knoten
];

fn beispiel_3_msg() -> Message<Fahrpult, 2> {
    Message {
        verbindungsaufbau: None,
        message: Some(Fahrpult {
            needed_data: Some(NeededData {
                fuehrerstands_anzeigen: Some(FuehrerstandsAnzeigen {
                    anzeigen: vec![0x0001, 0x001B],
                }),
                fuehrerstands_bedienung: None,
                programmdaten: None,
            }),
            ..Default::default()
        }),
    }
}

static BEISPIEL_4_BYTES: &'static [u8] = &[
    0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x03, 0x00, 0x00, 0x00,
    0x01, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
];

fn beispiel_4_msg() -> Message<Fahrpult, 2> {
    Message {
        message: Some(Fahrpult {
            ack_needed_data: Some(AckNeededData { error_code: 0 }),
            ..Default::default()
        }),
        ..Default::default()
    }
}

static BEISPIEL_5_BYTES: &'static [u8] = &[
    0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0A, 0x00, 0x06, 0x00, 0x00, 0x00,
    0x01, 0x00, 0xAE, 0x47, 0x3D, 0x41, 0x06, 0x00, 0x00, 0x00, 0x1B, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
];

fn beispiel_5_msg() -> Message<Fahrpult, 2> {
    Message {
        message: Some(Fahrpult {
            data_ftd: Some(DataFtd {
                geschwindigkeit: Some(11.83),
                lm_schleudern: Some(0.),
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    }
}

#[test]
fn test_beispiel_1_serialize() {
    let msg = beispiel_1_msg();

    let mut result: Vec<u8> = Default::default();
    send_verbindungsaufbau(msg.verbindungsaufbau.unwrap(), &mut result).unwrap();

    if result != BEISPIEL_1_BYTES {
        panic!(
            r#"
Expected:
{:02X?}
Result:
{:02X?}
"#,
            BEISPIEL_1_BYTES, result
        )
    }
    // assert_eq!(result, BEISPIEL_1_BYTES);
}

#[test]
fn test_beispiel_1_deserialize() {
    let result: FahrpultMessage = receive_message(&mut &BEISPIEL_1_BYTES[..]).unwrap();

    assert_eq!(beispiel_1_msg(), result);
}

#[test]
fn test_beispiel_2_serialize() {
    let msg = beispiel_2_msg();

    let mut result: Vec<u8> = Default::default();
    send_verbindungsaufbau(msg.verbindungsaufbau.unwrap(), &mut result).unwrap();

    assert_eq!(result, BEISPIEL_2_BYTES);
}

#[test]
fn test_beispiel_2_deserialize() {
    let result: FahrpultMessage = receive_message(&mut &BEISPIEL_2_BYTES[..]).unwrap();

    assert_eq!(beispiel_2_msg(), result);
}

#[test]
fn test_beispiel_3_serialize() {
    let msg = beispiel_3_msg();

    let mut result: Vec<u8> = Default::default();
    send_fahrpult(msg.message.unwrap(), &mut result).unwrap();

    assert_eq!(result, BEISPIEL_3_BYTES);
}

#[test]
fn test_beispiel_3_deserialize() {
    let result: FahrpultMessage = receive_message(&mut &BEISPIEL_3_BYTES[..]).unwrap();

    assert_eq!(beispiel_3_msg(), result);
}

#[test]
fn test_beispiel_3_with_unknown_attribute_deserialize() {
    let result: FahrpultMessage =
        receive_message(&mut &BEISPIEL_3_BYTES_WITH_UNKNOWN_ATTRIBUTE[..]).unwrap();

    assert_eq!(beispiel_3_msg(), result);
}

#[test]
fn test_beispiel_3_with_unknown_node_deserialize() {
    let result: FahrpultMessage =
        receive_message(&mut &BEISPIEL_3_BYTES_WITH_UNKNOWN_NODE[..]).unwrap();

    assert_eq!(beispiel_3_msg(), result);
}

#[test]
fn test_beispiel_3_with_unknown_node_nested_deserialize() {
    let result: FahrpultMessage =
        receive_message(&mut &BEISPIEL_3_BYTES_WITH_UNKNOWN_NODE_NESTED[..]).unwrap();

    assert_eq!(beispiel_3_msg(), result);
}

#[test]
fn test_beispiel_4_serialize() {
    let msg = beispiel_4_msg();

    let mut result: Vec<u8> = Default::default();
    send_fahrpult(msg.message.unwrap(), &mut result).unwrap();

    assert_eq!(result, BEISPIEL_4_BYTES);
}

#[test]
fn test_beispiel_4_deserialize() {
    let result: FahrpultMessage = receive_message(&mut &BEISPIEL_4_BYTES[..]).unwrap();

    assert_eq!(beispiel_4_msg(), result);
}

#[test]
fn test_beispiel_5_serialize() {
    let msg = beispiel_5_msg();

    let mut result: Vec<u8> = Default::default();
    send_fahrpult(msg.message.unwrap(), &mut result).unwrap();

    assert_eq!(result, BEISPIEL_5_BYTES);
}

#[test]
fn test_beispiel_5_deserialize() {
    let result: FahrpultMessage = receive_message(&mut &BEISPIEL_5_BYTES[..]).unwrap();

    assert_eq!(beispiel_5_msg(), result);
}

#[test]
fn test_receive_all() {
    let mut msg: Vec<u8> = Default::default();
    msg.extend_from_slice(BEISPIEL_1_BYTES);
    msg.extend_from_slice(BEISPIEL_2_BYTES);
    msg.extend_from_slice(BEISPIEL_3_BYTES);
    msg.extend_from_slice(BEISPIEL_4_BYTES);
    msg.extend_from_slice(BEISPIEL_5_BYTES);
    let mut msg = Cursor::new(msg);

    let result: FahrpultMessage = receive_message(&mut msg).unwrap();
    assert_eq!(beispiel_1_msg(), result);

    let result: FahrpultMessage = receive_message(&mut msg).unwrap();
    assert_eq!(beispiel_2_msg(), result);

    let result: FahrpultMessage = receive_message(&mut msg).unwrap();
    assert_eq!(beispiel_3_msg(), result);

    let result: FahrpultMessage = receive_message(&mut msg).unwrap();
    assert_eq!(beispiel_4_msg(), result);

    let result: FahrpultMessage = receive_message(&mut msg).unwrap();
    assert_eq!(beispiel_5_msg(), result);

    let mut buf = vec![0u8, 8];
    let len = msg.read_to_end(&mut buf).unwrap();
    assert_eq!(len, 0, "should have read till the end");
}

#[test]
fn test_send_all() {
    let mut result: Cursor<Vec<u8>> = Default::default();

    let msg = beispiel_1_msg();
    send_verbindungsaufbau(msg.verbindungsaufbau.unwrap(), &mut result).unwrap();

    let msg = beispiel_2_msg();
    send_verbindungsaufbau(msg.verbindungsaufbau.unwrap(), &mut result).unwrap();

    let msg = beispiel_3_msg();
    send_fahrpult(msg.message.unwrap(), &mut result).unwrap();

    let msg = beispiel_4_msg();
    send_fahrpult(msg.message.unwrap(), &mut result).unwrap();

    let msg = beispiel_5_msg();
    send_fahrpult(msg.message.unwrap(), &mut result).unwrap();

    let mut expected: Vec<u8> = Default::default();
    expected.extend_from_slice(BEISPIEL_1_BYTES);
    expected.extend_from_slice(BEISPIEL_2_BYTES);
    expected.extend_from_slice(BEISPIEL_3_BYTES);
    expected.extend_from_slice(BEISPIEL_4_BYTES);
    expected.extend_from_slice(BEISPIEL_5_BYTES);

    assert_eq!(result.into_inner(), expected);
}
