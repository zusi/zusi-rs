/// This test contains the example from the Zusi 3 Documentation.
/// It serializes and deserializes each message and compares it to the documentation example.
mod common;
use common::*;

use std::io::{Cursor, Read};
use zusi::Message;

#[test]
fn test_beispiel_1_serialize() {
    let msg = beispiel_1_msg();

    let mut result: Vec<u8> = Default::default();
    Message::write(&msg, &mut result).unwrap();

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
    let result: FahrpultMessage = Message::receive(&mut &BEISPIEL_1_BYTES[..]).unwrap();

    assert_eq!(beispiel_1_msg(), result);
}

#[test]
fn test_beispiel_2_serialize() {
    let msg = beispiel_2_msg();

    let mut result: Vec<u8> = Default::default();
    Message::write(&msg, &mut result).unwrap();

    assert_eq!(result, BEISPIEL_2_BYTES);
}

#[test]
fn test_beispiel_2_deserialize() {
    let result: FahrpultMessage = Message::receive(&mut &BEISPIEL_2_BYTES[..]).unwrap();

    assert_eq!(beispiel_2_msg(), result);
}

#[test]
fn test_beispiel_3_serialize() {
    let msg = beispiel_3_msg();

    let mut result: Vec<u8> = Default::default();
    Message::write(&msg, &mut result).unwrap();

    assert_eq!(result, BEISPIEL_3_BYTES);
}

#[test]
fn test_beispiel_3_deserialize() {
    let result: FahrpultMessage = Message::receive(&mut &BEISPIEL_3_BYTES[..]).unwrap();

    assert_eq!(beispiel_3_msg(), result);
}

#[test]
fn test_beispiel_3_with_unknown_attribute_deserialize() {
    let result: FahrpultMessage =
        Message::receive(&mut &BEISPIEL_3_BYTES_WITH_UNKNOWN_ATTRIBUTE[..]).unwrap();

    assert_eq!(beispiel_3_msg(), result);
}

#[test]
fn test_beispiel_3_with_unknown_node_deserialize() {
    let result: FahrpultMessage =
        Message::receive(&mut &BEISPIEL_3_BYTES_WITH_UNKNOWN_NODE[..]).unwrap();

    assert_eq!(beispiel_3_msg(), result);
}

#[test]
fn test_beispiel_3_with_unknown_node_nested_deserialize() {
    let result: FahrpultMessage =
        Message::receive(&mut &BEISPIEL_3_BYTES_WITH_UNKNOWN_NODE_NESTED[..]).unwrap();

    assert_eq!(beispiel_3_msg(), result);
}

#[test]
fn test_beispiel_4_serialize() {
    let msg = beispiel_4_msg();

    let mut result: Vec<u8> = Default::default();
    Message::write(&msg, &mut result).unwrap();

    assert_eq!(result, BEISPIEL_4_BYTES);
}

#[test]
fn test_beispiel_4_deserialize() {
    let result: FahrpultMessage = Message::receive(&mut &BEISPIEL_4_BYTES[..]).unwrap();

    assert_eq!(beispiel_4_msg(), result);
}

#[test]
fn test_beispiel_5_serialize() {
    let msg = beispiel_5_msg();

    let mut result: Vec<u8> = Default::default();
    Message::write(&msg, &mut result).unwrap();

    assert_eq!(result, BEISPIEL_5_BYTES);
}

#[test]
fn test_beispiel_5_deserialize() {
    let result: FahrpultMessage = Message::receive(&mut &BEISPIEL_5_BYTES[..]).unwrap();

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

    let result: FahrpultMessage = Message::receive(&mut msg).unwrap();
    assert_eq!(beispiel_1_msg(), result);

    let result: FahrpultMessage = Message::receive(&mut msg).unwrap();
    assert_eq!(beispiel_2_msg(), result);

    let result: FahrpultMessage = Message::receive(&mut msg).unwrap();
    assert_eq!(beispiel_3_msg(), result);

    let result: FahrpultMessage = Message::receive(&mut msg).unwrap();
    assert_eq!(beispiel_4_msg(), result);

    let result: FahrpultMessage = Message::receive(&mut msg).unwrap();
    assert_eq!(beispiel_5_msg(), result);

    let mut buf = vec![0u8, 8];
    let len = msg.read_to_end(&mut buf).unwrap();
    assert_eq!(len, 0, "should have read till the end");
}

#[test]
fn test_send_all() {
    let mut result: Cursor<Vec<u8>> = Default::default();

    let msg = beispiel_1_msg();
    Message::write(&msg, &mut result).unwrap();

    let msg = beispiel_2_msg();
    Message::write(&msg, &mut result).unwrap();

    let msg = beispiel_3_msg();
    Message::write(&msg, &mut result).unwrap();

    let msg = beispiel_4_msg();
    Message::write(&msg, &mut result).unwrap();

    let msg = beispiel_5_msg();
    Message::write(&msg, &mut result).unwrap();

    let mut expected: Vec<u8> = Default::default();
    expected.extend_from_slice(BEISPIEL_1_BYTES);
    expected.extend_from_slice(BEISPIEL_2_BYTES);
    expected.extend_from_slice(BEISPIEL_3_BYTES);
    expected.extend_from_slice(BEISPIEL_4_BYTES);
    expected.extend_from_slice(BEISPIEL_5_BYTES);

    assert_eq!(result.into_inner(), expected);
}
