use bytes::BytesMut;
use tokio_util::codec::{Decoder, Encoder};

use common::*;
use zusi::client::async_codec::ZusiProtocolCodec;
use zusi_fahrpult::Fahrpult;

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
    beispiel_4: (BEISPIEL_4_BYTES, beispiel_4_msg()),
    beispiel_5: (BEISPIEL_5_BYTES, beispiel_5_msg()),
}
