use zusi_protocol::{to_bytes, Deserialize, Serialize};

use crate::fahrpult::{AckNeededData, DataFtd, Fahrpult, FuehrerstandsAnzeigen, NeededData};
use crate::verbindungsaufbau::{AckHello, Hello, Verbindungsaufbau};
use crate::Message;

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

fn beispiel_1_msg() -> Message {
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
        fahrpult: None,
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

fn beispiel_2_msg() -> Message {
    Message {
        verbindungsaufbau: Some(Verbindungsaufbau {
            hello: None,
            ack_hello: Some(AckHello {
                zusi_version: "3.0.1.0".to_string(),
                zusi_verbindungsinfo: "0".to_string(),
                error_code: 0,
                fahrplan_start_zeit: Some(41390.5),
            }),
        }),
        fahrpult: None,
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

fn beispiel_3_msg() -> Message {
    Message {
        verbindungsaufbau: None,
        fahrpult: Some(Fahrpult {
            needed_data: Some(NeededData {
                fuehrerstands_anzeigen: Some(FuehrerstandsAnzeigen {
                    anzeigen: vec![0x0001, 0x001B],
                }),
                fuehrerstands_bedienung: None,
                programmdaten: None,
            }),
            ack_needed_data: None,
            data_ftd: None,
            data_prog: None,
            control: None,
        }),
    }
}

static BEISPIEL_4_BYTES: &'static [u8] = &[
    0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x03, 0x00, 0x00, 0x00,
    0x01, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
];

fn beispiel_4_msg() -> Message {
    Message {
        verbindungsaufbau: None,
        fahrpult: Some(Fahrpult {
            needed_data: None,
            ack_needed_data: Some(AckNeededData { error_code: 0 }),
            data_ftd: None,
            data_prog: None,
            control: None,
        }),
    }
}

static BEISPIEL_5_BYTES: &'static [u8] = &[
    0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0A, 0x00, 0x06, 0x00, 0x00, 0x00,
    0x01, 0x00, 0xAE, 0x47, 0x3D, 0x41, 0x06, 0x00, 0x00, 0x00, 0x1B, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
];

fn beispiel_5_msg() -> Message {
    Message {
        verbindungsaufbau: None,
        fahrpult: Some(Fahrpult {
            needed_data: None,
            ack_needed_data: None,
            data_ftd: Some(DataFtd {
                geschwindigkeit: Some(11.83),
                druck_hauptluftleitung: None,
                druck_bremszylinder: None,
                druck_hauptluftbehaelter: None,
                luftpresser_laeuft: None,
                luftstrom_fvb: None,
                luftstrom_zbv: None,
                luefter_an: None,
                zugkraft_gesamt: None,
                zugkraft_pro_achse: None,
                zugkraft_soll_gesamt: None,
                zugkraft_soll_pro_achse: None,
                oberstrom: None,
                fahrleitungsspannung: None,
                motordrehzahl: None,
                uhrzeit_stunde: None,
                uhrzeit_minute: None,
                uhrzeit_sekunde: None,
                hauptschalter: None,
                trennschuetz: None,
                fahrstufe: None,
                afb_sollgeschwindigkeit: None,
                druck_hilfsluftbehaelter: None,
                zurueckgelegter_gesamtweg: None,
                lm_getriebe: None,
                lm_schleudern: Some(0.),
                lm_gleiten: None,
                lm_mg_bremse: None,
                lm_hbremse: None,
                lm_rbremse: None,
                lm_hochabbremsung: None,
                lm_schnellbremsung: None,
                status_notbremssystem: None,
                lm_uhrzeit_digital: None,
                lm_drehzahlverstellung: None,
                lm_fahrtrichtung_vor: None,
                lm_fahrtrichtung_zurueck: None,
                lm_fahrtrichtung_m: None,
                motordrehmoment: None,
                motorlast_normiert: None,
                tunnel: None,
                schienenstoss_weiche: None,
                stahlbruecke: None,
                steinbruecke: None,
                xkoordinate: None,
                ykoordinate: None,
                zkoordinate: None,
                utm_referenzpunkt_xkm: None,
                utmr_eferenzpunkt_ykm: None,
                utm_zone: None,
                utm_zone2: None,
                afb_an: None,
                individuell01: None,
                individuell02: None,
                individuell03: None,
                individuell04: None,
                individuell05: None,
                individuell06: None,
                individuell07: None,
                individuell08: None,
                individuell09: None,
                individuell10: None,
                individuell11: None,
                individuell12: None,
                individuell13: None,
                individuell14: None,
                individuell15: None,
                individuell16: None,
                individuell17: None,
                individuell18: None,
                individuell19: None,
                individuell20: None,
                datum: None,
                gleiskruemmung: None,
                streckenhoechstgeschwindigkeit: None,
                zugkraftvorschlag_autopilot: None,
                beschleunigung_x: None,
                beschleunigung_y: None,
                beschleunigung_z: None,
                drehbeschleunigung_x_achse: None,
                drehbeschleunigung_y_achse: None,
                drehbeschleunigung_z_achse: None,
                stromabnehmer: None,
                lm_federspeicherbremse: None,
                zustand_federspeicherbremse: None,
                steuerwagen_lm_getriebe: None,
                steuerwagen_lm_schleudern: None,
                steuerwagen_lm_gleiten: None,
                steuerwagen_lm_hbremse: None,
                steuerwagen_lm_rbremse: None,
                steuerwagen_lm_drehzahlverstellung: None,
                druck_zweitbehaelter: None,
                geschwindigkeit_absolut: None,
                zug_ist_entgleist: None,
                kilometrierung_zugspitze: None,
                motorstrom: None,
                motorspannung: None,
                status_sifa: None,
                status_zugsicherung: None,
                status_tuersystem: None,
                individuell21: None,
                individuell22: None,
                individuell23: None,
                individuell24: None,
                individuell25: None,
                individuell26: None,
                individuell27: None,
                individuell28: None,
                individuell29: None,
                individuell30: None,
                individuell31: None,
                individuell32: None,
                individuell33: None,
                individuell34: None,
                individuell35: None,
                individuell36: None,
                individuell37: None,
                individuell38: None,
                individuell39: None,
                individuell40: None,
                steuerwagen_luefter_an: None,
                steuerwagen_zugkraft_gesamt: None,
                steuerwagen_zugraft_pro_achse: None,
                steuerwagen_zugkraft_soll_gesamt: None,
                steuerwagen_zugraft_soll_pro_achse: None,
                steuerwagen_oberstrom: None,
                steuerwagen_fahrleitungsspannung: None,
                steuerwagen_motordrehzahl: None,
                steuerwagen_hauptschalter: None,
                steuerwagen_trennschuetz: None,
                steuerwagen_fahrstufe: None,
                steuerwagen_motordrehmoment: None,
                steuerwagen_motorlast_normiert: None,
                steuerwagen_stromabnehmer: None,
                steuerwagen_motorstrom: None,
                steuerwagen_motorspannung: None,
                geschwindigkeit_absolut_inkl_schleudern: None,
                batteriehauptschalter_aus: None,
                status_fahrzeug: None,
                status_zugverband: None,
                bremsprobenfunktion: None,
                zug_und_brems_gesamtkraft_soll_normiert: None,
                steuerwagen_zug_und_brems_gesamtkraft_soll_normiert: None,
                status_weichen: None,
                zug_und_brems_gesamtkraft_absolut_normiert: None,
                steuerwagen_zug_und_brems_gesamtkraft_absolut_normiert: None,
            }),
            data_prog: None,
            control: None,
        }),
    }
}

#[test]
fn test_beispiel_1_serialize() {
    let msg = beispiel_1_msg();

    let result = to_bytes(&msg).unwrap();

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
    let result: Message = Message::deserialize(&mut &BEISPIEL_1_BYTES[..], 0).unwrap();

    assert_eq!(beispiel_1_msg(), result);
}

#[test]
fn test_beispiel_2_serialize() {
    let msg = beispiel_2_msg();

    let result = to_bytes(&msg).unwrap();

    assert_eq!(result, BEISPIEL_2_BYTES);
}

#[test]
fn test_beispiel_2_deserialize() {
    let result: Message = Message::deserialize(&mut &BEISPIEL_2_BYTES[..], 1).unwrap();

    assert_eq!(beispiel_2_msg(), result);
}

#[test]
fn test_beispiel_3_serialize() {
    let msg = beispiel_3_msg();

    let result = to_bytes(&msg).unwrap();

    assert_eq!(result, BEISPIEL_3_BYTES);
}

#[test]
fn test_beispiel_3_deserialize() {
    let result: Message = Message::deserialize(&mut &BEISPIEL_3_BYTES[..], 1).unwrap();

    assert_eq!(beispiel_3_msg(), result);
}

#[test]
fn test_beispiel_4_serialize() {
    let msg = beispiel_4_msg();

    let result = to_bytes(&msg).unwrap();

    assert_eq!(result, BEISPIEL_4_BYTES);
}

#[test]
fn test_beispiel_4_deserialize() {
    let result: Message = Message::deserialize(&mut &BEISPIEL_4_BYTES[..], 1).unwrap();

    assert_eq!(beispiel_4_msg(), result);
}

#[test]
fn test_beispiel_5_serialize() {
    let msg = beispiel_5_msg();

    let result = to_bytes(&msg).unwrap();

    assert_eq!(result, BEISPIEL_5_BYTES);
}

#[test]
fn test_beispiel_5_deserialize() {
    let result: Message = Message::deserialize(&mut &BEISPIEL_5_BYTES[..], 1).unwrap();

    assert_eq!(beispiel_5_msg(), result);
}
