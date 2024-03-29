use zusi::fahrpult::{AckNeededData, DataFtd, Fahrpult, FuehrerstandsAnzeigen, NeededData};
use zusi::verbindungsaufbau::{AckHello, Hello, Verbindungsaufbau};
use zusi::Message;

pub type FahrpultMessage = Message<Fahrpult>;

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

pub fn beispiel_1_msg() -> FahrpultMessage {
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

pub static BEISPIEL_2_BYTES: &[u8] = &[
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

pub fn beispiel_2_msg() -> FahrpultMessage {
    Verbindungsaufbau {
        hello: None,
        ack_hello: Some(AckHello {
            zusi_version: "3.0.1.0".to_string(),
            zusi_verbindungsinfo: "0".to_string(),
            error_code: 0,
            fahrplan_start_zeit: Some(41390.5),
            protokoll_version: None,
        }),
    }
    .into()
}

pub static BEISPIEL_3_BYTES: &[u8] = &[
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

pub static BEISPIEL_3_BYTES_WITH_UNKNOWN_ATTRIBUTE: &[u8] = &[
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

pub static BEISPIEL_3_BYTES_WITH_UNKNOWN_NODE: &[u8] = &[
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

pub static BEISPIEL_3_BYTES_WITH_UNKNOWN_NODE_NESTED: &[u8] = &[
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

pub fn beispiel_3_msg() -> FahrpultMessage {
    Fahrpult {
        needed_data: Some(NeededData {
            fuehrerstands_anzeigen: Some(FuehrerstandsAnzeigen { anzeigen: vec![0x0001, 0x001B] }),
            fuehrerstands_bedienung: None,
            programmdaten: None,
        }),
        ..Default::default()
    }
    .into()
}

pub static BEISPIEL_4_BYTES: &[u8] = &[
    0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x03, 0x00, 0x00, 0x00,
    0x01, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
];

pub fn beispiel_4_msg() -> FahrpultMessage {
    Fahrpult { ack_needed_data: Some(AckNeededData { error_code: 0 }), ..Default::default() }.into()
}

pub static BEISPIEL_5_BYTES: &[u8] = &[
    0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0A, 0x00, 0x06, 0x00, 0x00, 0x00,
    0x01, 0x00, 0xAE, 0x47, 0x3D, 0x41, 0x06, 0x00, 0x00, 0x00, 0x1B, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
];

pub fn beispiel_5_msg() -> FahrpultMessage {
    Fahrpult {
        data_ftd: Some(DataFtd {
            geschwindigkeit: Some(11.83),
            lm_schleudern: Some(0.),
            ..Default::default()
        }),
        ..Default::default()
    }
    .into()
}
