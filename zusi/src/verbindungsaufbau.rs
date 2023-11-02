use zusi_protocol::{Deserialize, Serialize};
use zusi_protocol_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
#[zusi(id = 0x0001)]
/// 5.3.2 Verbindungsaufbau
pub struct Verbindungsaufbau {
    #[zusi(id = 0x0001)]
    /// 5.3.2.1 Befehl `00 01` - HELLO (Client → Zusi)
    pub hello: Option<Hello>,
    #[zusi(id = 0x0002)]
    /// 5.3.2.2 Befehl `00 02` - ACK_HELLO (Zusi → Client)
    pub ack_hello: Option<AckHello>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
/// 5.3.2.1 Befehl `00 01` - HELLO (Client → Zusi)
pub struct Hello {
    #[zusi(id = 0x0001)]
    /// Protokoll-Version
    pub protokoll_version: u16,
    #[zusi(id = 0x0002)]
    /// Angabe des Client-Typs zur Unterscheidung, um welche Art von Anwendung es sich handelt:
    ///
    /// - 01: Zusi
    /// - 02: Fahrpult
    pub client_typ: u16,
    #[zusi(id = 0x0003)]
    /// Klartextstring zur Identifikation des Programms durch den Anwender.
    pub name: String,
    #[zusi(id = 0x0004)]
    /// Versionsnummer des Clients
    pub version: String,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
/// 5.3.2.2 Befehl `0002` - ACK_HELLO (Zusi → Client)
pub struct AckHello {
    #[zusi(id = 0x0001)]
    /// Zusi-Version
    pub zusi_version: String,
    #[zusi(id = 0x0002)]
    /// Zusi-Verbindungsinfo
    pub zusi_verbindungsinfo: String,
    #[zusi(id = 0x0003)]
    /// Der Client wurde akzeptiert, wenn das Byte auf `00` steht.
    ///
    /// Wird der Client nicht akzeptiert, wird stattdessen ein anderes Byte gesendet.
    /// Der Server bricht daraufhin die Verbindung ab.
    pub error_code: u8,
    #[zusi(id = 0x0004)]
    /// Startdatum und -zeit des geladenen Fahrplans in Tagen seit 30.12.1899.
    /// Ist kein Fahrplan geladen, wird 0 geschickt.
    pub fahrplan_start_zeit: Option<f64>,
    #[zusi(id = 0x0005)]
    /// Version des TCP-Protokolls
    pub protokoll_version: Option<String>,
}
