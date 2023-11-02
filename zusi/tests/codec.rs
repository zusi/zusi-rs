use bytes::BytesMut;
use tokio_util::codec::{Decoder, Encoder};

use common::*;
use zusi::client::ZusiProtocolCodec;
use zusi::fahrpult::Fahrpult;

mod common;

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
    let mut codec = ZusiProtocolCodec::<Fahrpult>::new();
    let mut output = BytesMut::new();
    codec.encode(msg, &mut output).unwrap();

    if output != bts {
        panic!(
            r#"
Expected:
{:02X?}
Result:
{:02X?}
"#,
            bts, output
        )
    }
}

fn deserialize(bts: &[u8], msg: FahrpultMessage) {
    let mut decoder = ZusiProtocolCodec::<Fahrpult>::new();
    let mut bts = BytesMut::from(bts);
    let result = decoder.decode(&mut bts).unwrap();

    assert_ne!(result, None);
    assert_eq!(result.unwrap(), msg);
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
    let mut bts: Vec<u8> = Default::default();
    bts.extend_from_slice(BEISPIEL_1_BYTES);
    bts.extend_from_slice(BEISPIEL_2_BYTES);
    bts.extend_from_slice(BEISPIEL_3_BYTES);
    bts.extend_from_slice(BEISPIEL_4_BYTES);
    bts.extend_from_slice(BEISPIEL_5_BYTES);

    let mut decoder = ZusiProtocolCodec::<Fahrpult>::new();
    let mut bts = BytesMut::from(&bts[..]);

    fn receive_one(
        decoder: &mut ZusiProtocolCodec<Fahrpult>,
        src: &mut BytesMut,
    ) -> Option<FahrpultMessage> {
        decoder.decode(src).unwrap()
    }

    assert_eq!(beispiel_1_msg(), receive_one(&mut decoder, &mut bts).unwrap());
    assert_eq!(beispiel_2_msg(), receive_one(&mut decoder, &mut bts).unwrap());
    assert_eq!(beispiel_3_msg(), receive_one(&mut decoder, &mut bts).unwrap());
    assert_eq!(beispiel_4_msg(), receive_one(&mut decoder, &mut bts).unwrap());
    assert_eq!(beispiel_5_msg(), receive_one(&mut decoder, &mut bts).unwrap());

    // we read all messages, so there should be no more
    assert_eq!(None, receive_one(&mut decoder, &mut bts));
    assert_eq!(bts.len(), 0, "should have read till the end");
}

#[test]
fn test_send_all() {
    let mut codec = ZusiProtocolCodec::<Fahrpult>::new();
    let mut output = BytesMut::new();

    codec.encode(beispiel_1_msg(), &mut output).unwrap();
    codec.encode(beispiel_2_msg(), &mut output).unwrap();
    codec.encode(beispiel_3_msg(), &mut output).unwrap();
    codec.encode(beispiel_4_msg(), &mut output).unwrap();
    codec.encode(beispiel_5_msg(), &mut output).unwrap();

    let mut expected: Vec<u8> = Default::default();
    expected.extend_from_slice(BEISPIEL_1_BYTES);
    expected.extend_from_slice(BEISPIEL_2_BYTES);
    expected.extend_from_slice(BEISPIEL_3_BYTES);
    expected.extend_from_slice(BEISPIEL_4_BYTES);
    expected.extend_from_slice(BEISPIEL_5_BYTES);

    assert_eq!(output, expected);
}
