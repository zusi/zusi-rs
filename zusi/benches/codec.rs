use bytes::BytesMut;
use divan::{black_box, AllocProfiler, Bencher};
use tokio_util::codec::{Decoder, Encoder};

use zusi::client::ZusiProtocolCodec;
use zusi::fahrpult::Fahrpult;
use zusi::verbindungsaufbau::{Hello, Verbindungsaufbau};
use zusi::Message;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

pub fn beispiel_1_msg() -> Message<Fahrpult> {
    Verbindungsaufbau {
        hello: Some(Hello {
            protokoll_version: 2,
            client_typ: 2,
            name: "Fahrpult".to_string(),
            version: "2.0".to_string(),
        }),
        ack_hello: None,
    }
    .into()
}

pub static BEISPIEL_1_BYTES: &[u8] = &[
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

fn main() {
    divan::main();
}

#[divan::bench]
fn encode_message(bencher: Bencher) {
    let mut codec = ZusiProtocolCodec::<Fahrpult>::default();
    let mut output = BytesMut::with_capacity(1024 * 1024);
    bencher.bench_local(|| codec.encode(black_box(beispiel_1_msg()), &mut output));
}

#[divan::bench]
fn decode_message(bencher: Bencher) {
    let mut codec = ZusiProtocolCodec::<Fahrpult>::default();
    let mut bts = BytesMut::from(BEISPIEL_1_BYTES);
    bencher.bench_local(|| codec.decode(black_box(&mut bts)));
}
