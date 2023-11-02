/// This test contains the example from the Zusi 3 Documentation.
/// It serializes and deserializes each message and compares it to the documentation example.
mod common;
use common::*;

use std::io::{Cursor, Read};
use zusi::Message;

macro_rules! serialize_tests {
    ($($name:ident: ($bts:expr,$msg:expr),)*) => {
        mod serialize {
            use crate::common::*;
            $(
                #[test]
                fn $name() { super::serialize($bts,$msg); }
            )*
        }
    };
}

macro_rules! deserialize_tests {
    ($($name:ident: ($bts:expr,$msg:expr),)*) => {
        mod deserialize {
            use crate::common::*;
            $(
                #[test]
                fn $name() { super::deserialize($bts,$msg); }
            )*
        }
    };
}

fn serialize(bts: &[u8], msg: FahrpultMessage) {
    let mut result: Vec<u8> = Default::default();
    Message::write(&msg, &mut result).unwrap();

    if result != bts {
        panic!(
            r#"
Expected:
{:02X?}
Result:
{:02X?}
"#,
            bts, result
        )
    }
}

fn deserialize(mut bts: &[u8], msg: FahrpultMessage) {
    let result: FahrpultMessage = Message::receive(&mut bts).unwrap();

    assert_eq!(msg, result);
}
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

serialize_tests! {
    beispiel_1: (BEISPIEL_1_BYTES, beispiel_1_msg()),
    beispiel_2: (BEISPIEL_2_BYTES, beispiel_2_msg()),
    beispiel_3: (BEISPIEL_3_BYTES, beispiel_3_msg()),
    beispiel_4: (BEISPIEL_4_BYTES, beispiel_4_msg()),
    beispiel_5: (BEISPIEL_5_BYTES, beispiel_5_msg()),
}

deserialize_tests! {
    beispiel_1: (BEISPIEL_1_BYTES, beispiel_1_msg()),
    beispiel_2: (BEISPIEL_2_BYTES, beispiel_2_msg()),
    beispiel_3: (BEISPIEL_3_BYTES, beispiel_3_msg()),
    beispiel_3_with_unknown_attribute: (BEISPIEL_3_BYTES_WITH_UNKNOWN_ATTRIBUTE, beispiel_3_msg()),
    beispiel_3_with_unknown_node: (BEISPIEL_3_BYTES_WITH_UNKNOWN_NODE, beispiel_3_msg()),
    beispiel_3_with_unknown_node_nested: (BEISPIEL_3_BYTES_WITH_UNKNOWN_NODE_NESTED, beispiel_3_msg()),
    beispiel_4: (BEISPIEL_4_BYTES, beispiel_4_msg()),
    beispiel_5: (BEISPIEL_5_BYTES, beispiel_5_msg()),
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
