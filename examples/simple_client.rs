use zusi_protocol::{to_bytes, Serialize};
use zusi_protocol_derive::Serialize;

#[derive(Serialize)]
struct HelloMsg {
    #[zusi(id = 0x0001)]
    protokoll_version: u16,
    #[zusi(id = 0x0002)]
    client_typ: u16,
    #[zusi(id = 0x0003)]
    name: String,
    #[zusi(id = 0x0004)]
    version: String,
}

fn main() {
    let hello = HelloMsg {
        protokoll_version: 2,
        client_typ: 2,
        name: "Fahrpult".to_string(),
        version: "2.0".to_string(),
    };

    let bts = to_bytes(&hello).unwrap();

    let expected: Vec<u8> = vec![
        0x00, 0x00, 0x00, 0x00, // Länge 0 Bytes → es beginnt ein Knoten
        0x01, 0x00, // ID 1: Verbindungsaufbau
        0x04, 0x00, 0x00, 0x00, // Länge 4 Bytes → es folgt ein Attribut, Länge 4 bytes
        0x01, 0x00, // ID x0001: Protokoll-Version
        0x02, 0x00, // Protokoll-Version "2" (Word)
        0x04, 0x00, 0x00, 0x00, // Länge 4 Bytes → es folgt ein Attribut, Länge 4 bytes
        0x02, 0x00, // ID x0002: Client-Typ
        0x02, 0x00, // Client-Typ "Fahrpult" (Word)
        0x0A, 0x00, 0x00, 0x00, // Länge 10 Bytes → es folgt ein Attribut
        0x03, 0x00, // ID x0003: Klartextstring
        0x46, 0x61, 0x68, 0x72, 0x70, 0x75, 0x6C,
        0x74, // String "Fahrpult" (8 Zeichen, da 2 bytes für die ID)
        0x05, 0x00, 0x00, 0x00, // Länge 5 Bytes → es folgt ein Attribut
        0x04, 0x00, // ID x0004: Version
        0x32, 0x2E, 0x30, // String "2.0"
        0xFF, 0xFF, 0xFF, 0xFF, // Ende Knoten
    ];
    assert_eq!(bts, expected);
}
