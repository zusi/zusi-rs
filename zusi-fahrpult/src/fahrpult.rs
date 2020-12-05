use zusi_protocol::{Deserialize, Serialize};
use zusi_protocol_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
#[zusi(id = 0x0002)]
/// Anwendung 02 ("Fahrpult")
pub struct Fahrpult {
    #[zusi(id = 0x0003)]
    /// 5.3.3.1 Befehl 00 03 - NEEDED_DATA (Client → Zusi)
    pub needed_data: Option<NeededData>,
    #[zusi(id = 0x0004)]
    /// 5.3.3.2 Befehl 00 04 - ACK_NEEDED_DATA (Zusi → Client)
    pub ack_needed_data: Option<AckNeededData>,
    #[zusi(id = 0x000A)]
    /// 5.3.3.3 Befehl 00 0A - DATA_FTD (Zusi → Client)
    pub data_ftd: Option<DataFtd>,
    #[zusi(id = 0x000C)]
    /// 5.3.3.5 Befehl 00 0C - DATA_PROG (Zusi → Client)
    pub data_prog: Option<DataProg>,
    #[zusi(id = 0x000B)]
    /// 5.3.3.7 Befehl 01 0B - CONTROL (Client → Zusi)
    pub control: Option<Control>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
/// 5.3.3.1 Befehl 00 03 - NEEDED_DATA (Client → Zusi)
pub struct NeededData {
    #[zusi(id = 0x000A)]
    pub fuehrerstands_anzeigen: Option<FuehrerstandsAnzeigen>,
    #[zusi(id = 0x000B)]
    pub fuehrerstands_bedienung: Option<FuehrerstandsBedienung>,
    #[zusi(id = 0x000C)]
    pub programmdaten: Option<Programmdaten>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct FuehrerstandsAnzeigen {
    #[zusi(id = 0x0001)]
    pub anzeigen: Vec<u16>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct FuehrerstandsBedienung {}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct Programmdaten {
    #[zusi(id = 0x0001)]
    pub anzeigen: Vec<u16>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
/// 5.3.3.2 Befehl 00 04 - ACK_NEEDED_DATA (Zusi → Client)
pub struct AckNeededData {
    #[zusi(id = 0x0001)]
    pub error_code: u8,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
/// 5.3.3.3 Befehl 00 0A - DATA_FTD (Zusi → Client)
pub struct DataFtd {
    #[zusi(id = 0x0001)]
    /// Single: m/s
    pub geschwindigkeit: Option<f32>,
    #[zusi(id = 0x0002)]
    /// Single: bar
    pub druck_hauptluftleitung: Option<f32>,
    #[zusi(id = 0x0003)]
    /// Single: bar
    pub druck_bremszylinder: Option<f32>,
    #[zusi(id = 0x0004)]
    /// Single: bar
    pub druck_hauptluftbehaelter: Option<f32>,
    #[zusi(id = 0x0005)]
    /// Single: aus/an
    pub luftpresser_laeuft: Option<f32>,
    #[zusi(id = 0x0006)]
    /// Single: -1..0..+1
    pub luftstrom_fvb: Option<f32>,
    #[zusi(id = 0x0007)]
    /// Single: -1..0..+1
    pub luftstrom_zbv: Option<f32>,
    #[zusi(id = 0x0008)]
    /// Single: aus/an
    pub luefter_an: Option<f32>,
    #[zusi(id = 0x0009)]
    /// Single: N
    pub zugkraft_gesamt: Option<f32>,
    #[zusi(id = 0x000A)]
    /// Single: N
    pub zugkraft_pro_achse: Option<f32>,
    #[zusi(id = 0x000B)]
    /// Single: N
    pub zugkraft_soll_gesamt: Option<f32>,
    #[zusi(id = 0x000C)]
    /// Single: N
    pub zugkraft_soll_pro_achse: Option<f32>,
    #[zusi(id = 0x000D)]
    /// Single: A
    pub oberstrom: Option<f32>,
    #[zusi(id = 0x000E)]
    /// Single: V
    pub fahrleitungsspannung: Option<f32>,
    #[zusi(id = 0x000F)]
    /// Single: 1/min
    pub motordrehzahl: Option<f32>,
    #[zusi(id = 0x0010)]
    /// Single: Stunde (Zeigerposition Analoguhren)
    pub uhrzeit_stunde: Option<f32>,
    #[zusi(id = 0x0011)]
    /// Single: Minute (Zeigerposition Analoguhren)
    pub uhrzeit_minute: Option<f32>,
    #[zusi(id = 0x0012)]
    /// Single: Sekunde (Zeigerposition Analoguhren)
    pub uhrzeit_sekunde: Option<f32>,
    #[zusi(id = 0x0013)]
    /// Single: aus/an
    pub hauptschalter: Option<f32>,
    #[zusi(id = 0x0014)]
    /// Single: aus/an
    pub trennschuetz: Option<f32>,
    #[zusi(id = 0x0015)]
    // Single: 1
    pub fahrstufe: Option<f32>,
    //#[zusi(id = 0x0016)]
    //d3DFenster                                       : Option<f32>, // Single: Nicht sinnvoll zu verwenden
    #[zusi(id = 0x0017)]
    /// Single: m/s
    pub afb_sollgeschwindigkeit: Option<f32>,
    #[zusi(id = 0x0018)]
    /// Single: bar
    pub druck_hilfsluftbehaelter: Option<f32>,
    #[zusi(id = 0x0019)]
    /// Single: m
    pub zurueckgelegter_gesamtweg: Option<f32>,
    #[zusi(id = 0x001A)]
    /// Single: aus/an
    pub lm_getriebe: Option<f32>,
    #[zusi(id = 0x001B)]
    /// Single: aus/an
    pub lm_schleudern: Option<f32>,
    #[zusi(id = 0x001C)]
    /// Single: aus/an
    pub lm_gleiten: Option<f32>,
    #[zusi(id = 0x001D)]
    /// Single: aus/an
    pub lm_mg_bremse: Option<f32>,
    #[zusi(id = 0x001E)]
    /// Single: aus/an
    pub lm_hbremse: Option<f32>,
    #[zusi(id = 0x001F)]
    /// Single: aus/an
    pub lm_rbremse: Option<f32>,
    #[zusi(id = 0x0020)]
    /// Single: aus/an
    pub lm_hochabbremsung: Option<f32>,
    #[zusi(id = 0x0021)]
    /// Single: aus/an
    pub lm_schnellbremsung: Option<f32>,
    #[zusi(id = 0x0022)]
    pub status_notbremssystem: Option<StatusNotbremssystem>,
    #[zusi(id = 0x0023)]
    /// Single: 0...1 (0:00 bis 24:00 Uhr)
    pub lm_uhrzeit_digital: Option<f32>,
    #[zusi(id = 0x0024)]
    /// Single: aus/an
    pub lm_drehzahlverstellung: Option<f32>,
    #[zusi(id = 0x0025)]
    /// Single: aus/an
    pub lm_fahrtrichtung_vor: Option<f32>,
    #[zusi(id = 0x0026)]
    /// Single: aus/an
    pub lm_fahrtrichtung_zurueck: Option<f32>,
    #[zusi(id = 0x0027)]
    /// Single: aus/an
    pub lm_fahrtrichtung_m: Option<f32>,
    // #[zusi(id = 0x0028)]
    // hintergrundbild: Option<f32>, // Single: Nicht sinnvoll zu verwenden
    #[zusi(id = 0x0029)]
    /// Single: Nm
    pub motordrehmoment: Option<f32>,
    #[zusi(id = 0x002A)]
    /// Single: 1 (0...1)
    pub motorlast_normiert: Option<f32>,
    #[zusi(id = 0x002B)]
    /// Single: aus/an
    pub tunnel: Option<f32>,
    #[zusi(id = 0x002C)]
    /// Single: aus/an
    pub schienenstoss_weiche: Option<f32>,
    #[zusi(id = 0x002D)]
    /// Single: aus/an
    pub stahlbruecke: Option<f32>,
    #[zusi(id = 0x002E)]
    /// Single: aus/an
    pub steinbruecke: Option<f32>,
    #[zusi(id = 0x002F)]
    /// m (Bez. Strecken-UTM-Punkt)
    pub xkoordinate: Option<f32>,
    #[zusi(id = 0x0030)]
    /// m (Bez. Strecken-UTM-Punkt)
    pub ykoordinate: Option<f32>,
    #[zusi(id = 0x0031)]
    /// m
    pub zkoordinate: Option<f32>,
    #[zusi(id = 0x0032)]
    /// km
    pub utm_referenzpunkt_xkm: Option<f32>,
    #[zusi(id = 0x0033)]
    /// km
    pub utmr_eferenzpunkt_ykm: Option<f32>,
    #[zusi(id = 0x0034)]
    pub utm_zone: Option<f32>,
    #[zusi(id = 0x0035)]
    pub utm_zone2: Option<f32>,
    #[zusi(id = 0x0036)]
    /// Single: aus/an
    pub afb_an: Option<f32>,
    #[zusi(id = 0x0037)]
    pub individuell01: Option<f32>,
    #[zusi(id = 0x0038)]
    pub individuell02: Option<f32>,
    #[zusi(id = 0x0039)]
    pub individuell03: Option<f32>,
    #[zusi(id = 0x003A)]
    pub individuell04: Option<f32>,
    #[zusi(id = 0x003B)]
    pub individuell05: Option<f32>,
    #[zusi(id = 0x003C)]
    pub individuell06: Option<f32>,
    #[zusi(id = 0x003D)]
    pub individuell07: Option<f32>,
    #[zusi(id = 0x003E)]
    pub individuell08: Option<f32>,
    #[zusi(id = 0x003F)]
    pub individuell09: Option<f32>,
    #[zusi(id = 0x0040)]
    pub individuell10: Option<f32>,
    #[zusi(id = 0x0041)]
    pub individuell11: Option<f32>,
    #[zusi(id = 0x0042)]
    pub individuell12: Option<f32>,
    #[zusi(id = 0x0043)]
    pub individuell13: Option<f32>,
    #[zusi(id = 0x0044)]
    pub individuell14: Option<f32>,
    #[zusi(id = 0x0045)]
    pub individuell15: Option<f32>,
    #[zusi(id = 0x0046)]
    pub individuell16: Option<f32>,
    #[zusi(id = 0x0047)]
    pub individuell17: Option<f32>,
    #[zusi(id = 0x0048)]
    pub individuell18: Option<f32>,
    #[zusi(id = 0x0049)]
    pub individuell19: Option<f32>,
    #[zusi(id = 0x004A)]
    pub individuell20: Option<f32>,
    #[zusi(id = 0x004B)]
    /// Single (Tage mit 0 = 30.12.1899)
    pub datum: Option<f32>,
    #[zusi(id = 0x004C)]
    /// Single 1000/m
    pub gleiskruemmung: Option<f32>,
    #[zusi(id = 0x004D)]
    /// Single: m/s
    pub streckenhoechstgeschwindigkeit: Option<f32>,
    #[zusi(id = 0x004E)]
    /// Single: N
    pub zugkraftvorschlag_autopilot: Option<f32>,
    #[zusi(id = 0x004F)]
    /// Single: m/s^2
    pub beschleunigung_x: Option<f32>,
    #[zusi(id = 0x0050)]
    /// Single: m/s^2
    pub beschleunigung_y: Option<f32>,
    #[zusi(id = 0x0051)]
    /// Single: m/s^2
    pub beschleunigung_z: Option<f32>,
    #[zusi(id = 0x0052)]
    /// Single: rad/s^2
    pub drehbeschleunigung_x_achse: Option<f32>,
    #[zusi(id = 0x0053)]
    /// Single: rad/s^2
    pub drehbeschleunigung_y_achse: Option<f32>,
    #[zusi(id = 0x0054)]
    /// Single: rad/s^2
    pub drehbeschleunigung_z_achse: Option<f32>,
    #[zusi(id = 0x0055)]
    /// Single: 2x4 bit (4bit: 1 fuer SA=oben; 4 bit: 1 fuer SA hebt sich gerade)
    pub stromabnehmer: Option<f32>,
    #[zusi(id = 0x0056)]
    /// Single: aus/an
    pub lm_federspeicherbremse: Option<f32>,
    #[zusi(id = 0x0057)]
    /// Single -1, 0, 1, 2 (nicht vorhanden, aus, an, blinkend)
    pub zustand_federspeicherbremse: Option<f32>,
    #[zusi(id = 0x0058)]
    /// Single: aus/an
    pub steuerwagen_lm_getriebe: Option<f32>,
    #[zusi(id = 0x0059)]
    /// Single: aus/an
    pub steuerwagen_lm_schleudern: Option<f32>,
    #[zusi(id = 0x005A)]
    /// Single: aus/an
    pub steuerwagen_lm_gleiten: Option<f32>,
    #[zusi(id = 0x005B)]
    /// Single: aus/an
    pub steuerwagen_lm_hbremse: Option<f32>,
    #[zusi(id = 0x005C)]
    /// Single: aus/an
    pub steuerwagen_lm_rbremse: Option<f32>,
    #[zusi(id = 0x005D)]
    /// Single: aus/an
    pub steuerwagen_lm_drehzahlverstellung: Option<f32>,
    #[zusi(id = 0x005E)]
    /// Single: bar
    pub druck_zweitbehaelter: Option<f32>,
    #[zusi(id = 0x005F)]
    /// Single: m/s
    pub geschwindigkeit_absolut: Option<f32>,
    #[zusi(id = 0x0060)]
    /// Single: aus/an
    pub zug_ist_entgleist: Option<f32>,
    #[zusi(id = 0x0061)]
    /// Single: km
    pub kilometrierung_zugspitze: Option<f32>,
    #[zusi(id = 0x0062)]
    /// Single: A
    pub motorstrom: Option<f32>,
    #[zusi(id = 0x0063)]
    /// Single: V
    pub motorspannung: Option<f32>,
    #[zusi(id = 0x0064)]
    pub status_sifa: Option<StatusSifa>,
    #[zusi(id = 0x0065)]
    pub status_zugsicherung: Option<StatusZugbeeinflussung>,
    #[zusi(id = 0x0066)]
    pub status_tuersystem: Option<StatusTuersystem>,
    #[zusi(id = 0x0067)]
    pub individuell21: Option<f32>,
    #[zusi(id = 0x0068)]
    pub individuell22: Option<f32>,
    #[zusi(id = 0x0069)]
    pub individuell23: Option<f32>,
    #[zusi(id = 0x006A)]
    pub individuell24: Option<f32>,
    #[zusi(id = 0x006B)]
    pub individuell25: Option<f32>,
    #[zusi(id = 0x006C)]
    pub individuell26: Option<f32>,
    #[zusi(id = 0x006D)]
    pub individuell27: Option<f32>,
    #[zusi(id = 0x006E)]
    pub individuell28: Option<f32>,
    #[zusi(id = 0x006F)]
    pub individuell29: Option<f32>,
    #[zusi(id = 0x0070)]
    pub individuell30: Option<f32>,
    #[zusi(id = 0x0071)]
    pub individuell31: Option<f32>,
    #[zusi(id = 0x0072)]
    pub individuell32: Option<f32>,
    #[zusi(id = 0x0073)]
    pub individuell33: Option<f32>,
    #[zusi(id = 0x0074)]
    pub individuell34: Option<f32>,
    #[zusi(id = 0x0075)]
    pub individuell35: Option<f32>,
    #[zusi(id = 0x0076)]
    pub individuell36: Option<f32>,
    #[zusi(id = 0x0077)]
    pub individuell37: Option<f32>,
    #[zusi(id = 0x0078)]
    pub individuell38: Option<f32>,
    #[zusi(id = 0x0079)]
    pub individuell39: Option<f32>,
    #[zusi(id = 0x007A)]
    pub individuell40: Option<f32>,
    #[zusi(id = 0x007B)]
    /// Single: aus/an
    pub steuerwagen_luefter_an: Option<f32>,
    #[zusi(id = 0x007C)]
    /// Single: N
    pub steuerwagen_zugkraft_gesamt: Option<f32>,
    #[zusi(id = 0x007D)]
    /// Single: N
    pub steuerwagen_zugraft_pro_achse: Option<f32>,
    #[zusi(id = 0x007E)]
    /// Single: N
    pub steuerwagen_zugkraft_soll_gesamt: Option<f32>,
    #[zusi(id = 0x007F)]
    /// Single: N
    pub steuerwagen_zugraft_soll_pro_achse: Option<f32>,
    #[zusi(id = 0x0080)]
    /// Single: N
    pub steuerwagen_oberstrom: Option<f32>,
    #[zusi(id = 0x0081)]
    /// Single V
    pub steuerwagen_fahrleitungsspannung: Option<f32>,
    #[zusi(id = 0x0082)]
    /// Single 1/min
    pub steuerwagen_motordrehzahl: Option<f32>,
    #[zusi(id = 0x0083)]
    /// Single aus/an
    pub steuerwagen_hauptschalter: Option<f32>,
    #[zusi(id = 0x0084)]
    /// Single: aus/an
    pub steuerwagen_trennschuetz: Option<f32>,
    #[zusi(id = 0x0085)]
    /// Single: 1
    pub steuerwagen_fahrstufe: Option<f32>,
    #[zusi(id = 0x0086)]
    /// Single: Nm
    pub steuerwagen_motordrehmoment: Option<f32>,
    #[zusi(id = 0x0087)]
    /// Single: 1 (0...1)
    pub steuerwagen_motorlast_normiert: Option<f32>,
    #[zusi(id = 0x0088)]
    /// Single
    pub steuerwagen_stromabnehmer: Option<f32>,
    #[zusi(id = 0x0089)]
    /// Single: A
    pub steuerwagen_motorstrom: Option<f32>,
    #[zusi(id = 0x008A)]
    /// Single: V
    pub steuerwagen_motorspannung: Option<f32>,
    #[zusi(id = 0x008B)]
    /// Single: m/s
    pub geschwindigkeit_absolut_inkl_schleudern: Option<f32>,
    #[zusi(id = 0x008C)]
    /// Single: aus/an
    pub batteriehauptschalter_aus: Option<f32>,
    #[zusi(id = 0x008D)]
    pub status_fahrzeug: Option<StatusFahrzeug>,
    #[zusi(id = 0x008E)]
    pub status_zugverband: Option<StatusZugverband>,
    #[zusi(id = 0x008F)]
    /// Single: 0 (aus, >0 aktiv)
    pub bremsprobenfunktion: Option<f32>,
    #[zusi(id = 0x0090)]
    /// Single: 1 ((0...1) normiert auf aktuelle Fmax
    pub zug_und_brems_gesamtkraft_soll_normiert: Option<f32>,
    #[zusi(id = 0x0091)]
    /// Single: 1 ((0...1) normiert auf aktuelle Fmax
    pub steuerwagen_zug_und_brems_gesamtkraft_soll_normiert: Option<f32>,
    #[zusi(id = 0x0092)]
    pub status_weichen: Option<StatusWeichen>,
    #[zusi(id = 0x0093)]
    /// Single: 1 ((0...1) normiert auf aktuelle Fmax
    pub zug_und_brems_gesamtkraft_absolut_normiert: Option<f32>,
    #[zusi(id = 0x0094)]
    /// Single: 1 ((0...1) normiert auf aktuelle Fmax
    pub steuerwagen_zug_und_brems_gesamtkraft_absolut_normiert: Option<f32>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
/// 5.3.3.3.2
pub struct StatusNotbremssystem {
    #[zusi(id = 0x0001)]
    pub bauart: Option<String>,
    #[zusi(id = 0x0002)]
    /// 0: NBÜ aus 1: NBÜ bereit 2: Notbremse gezogen 3: Notbremse wirkt (NBÜ bereit) 4: NBÜ durch Lokführer aktiviert 5 Notbremse wirkt (NBÜ aus)
    pub status_notbremssystem: Option<u8>,
    #[zusi(id = 0x0003)]
    /// 1: an
    pub lm_system_bereit: Option<u8>,
    #[zusi(id = 0x0004)]
    pub lm_notbremsung: Option<u8>,
    #[zusi(id = 0x0005)]
    pub testmodus_aktiv: Option<u8>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
/// 5.3.3.3.3
pub struct StatusSifa {
    #[zusi(id = 0x0001)]
    pub bauart: Option<String>,
    #[zusi(id = 0x0002)]
    pub lm_sifa: Option<u8>,
    #[zusi(id = 0x0003)]
    pub sifa_hupe: Option<u8>,
    #[zusi(id = 0x0004)]
    pub sifa_hauptschalter: Option<u8>,
    #[zusi(id = 0x0005)]
    pub sifa_stoerschalter: Option<u8>,
    #[zusi(id = 0x0006)]
    pub sifa_luftabsperrhahn: Option<u8>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
/// 5.3.3.3.4 Status Zugbeeinflussung
pub struct StatusZugbeeinflussung {
    #[zusi(id = 0x0001)]
    pub bauart: Option<String>,
    #[zusi(id = 0x0002)]
    pub indusi_einstellungen: Option<IndusiEinstellungen>,
    #[zusi(id = 0x0003)]
    pub indusi_zustand: Option<IndusiZustand>,
    #[zusi(id = 0x0004)]
    pub etcs_einstellungen: Option<EtcsEinstellungen>,
    #[zusi(id = 0x0005)]
    pub etcs_betriebsdaten: Option<EtcsBetriebsdaten>,
    #[zusi(id = 0x0006)]
    pub zub_einstellungen: Option<ZubEinstellungen>,
    #[zusi(id = 0x0007)]
    pub zub_betriebsdaten: Option<ZubBetriebsdaten>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
/// 5.3.3.3.3.4.2 Indusi Analogsysteme und Basisdaten
pub struct IndusiEinstellungen {
    #[zusi(id = 0x0001)]
    /// Zugart:1: Zugart muss noch bestimmt werden 2: U 3: M 4: O 5: S-Bahn-Modus
    pub zugart: Option<u8>,
    #[zusi(id = 0x0002)]
    pub tf_nummer: Option<String>,
    #[zusi(id = 0x0003)]
    pub zugnummer: Option<String>,
    #[zusi(id = 0x0004)]
    pub grunddaten: Option<Zugdaten>, // LZB
    #[zusi(id = 0x0005)]
    pub ersatzzugdaten: Option<Zugdaten>,
    #[zusi(id = 0x0006)]
    pub aktive_zugdaten: Option<Zugdaten>,
    #[zusi(id = 0x0007)]
    /// Hauptschalter   1: Indusi ausgeschaltet 2: Indusi eingeschaltet
    pub hauptschalter: Option<u8>,
    #[zusi(id = 0x0008)]
    /// Störschalter    1: Indusi abgeschaltet  2: Indusi eingeschaltet
    pub pzb_stoerschalter: Option<u8>,
    #[zusi(id = 0x0009)]
    pub lzb_stoerschalter: Option<u8>,
    #[zusi(id = 0x000A)]
    /// Luftabsperrhahn 1: abgesperrt 2: offen
    pub luftabsperrhahn: Option<u8>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct Zugdaten {
    #[zusi(id = 0x0001)]
    pub bremshundertstel: Option<u16>,
    #[zusi(id = 0x0002)]
    pub bremsart: Option<u16>,
    #[zusi(id = 0x0003)]
    /// LZB
    pub zuglaenge: Option<u16>,
    #[zusi(id = 0x0004)]
    /// LZB in km/h
    pub vmax: Option<u16>,
    #[zusi(id = 0x0005)]
    pub zugart: Option<u8>,
    #[zusi(id = 0x0006)]
    /// Nur relevant für AktiveZugdaten. 5: Ersatzzugdaten 6: Normalbetrieb
    pub modus: Option<u8>,
    #[zusi(id = 0x000B)]
    /// 0: Keine Klartextmeldungen möglich 1: Keine Klartextmeldungen möglich aber nicht aktiv 2: Klartextmeldungen aktiv 3: nur Klartextmeldungen möglich
    pub klartextmeldungen: Option<u8>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct IndusiZustand {
    #[zusi(id = 0x0002)]
    /// 1: Ausgeschaltet 2: abgeschaltet/gestört (1000Hz blinkt) 3: Hauptluftleitung unter 2,2 bar (1000Hz blinkt) 4: Aufforderung zur Zugdateneingabe 5: Normalbetrieb 6: Funktionsprüfung
    pub zugsicherung: Option<u16>,
    #[zusi(id = 0x0003)]
    /// 0: keine Zwangsbremsung, sonst Zwansbremsung aktiv wegen: 1. Wachsam 2. 1000Hz 3. 500Hz 4. 2000Hz 5. Kein Halt nach Befreiung aus Zwangsbremsung 6. Fahrzeug-v-Max überschritten 7. Funktionsprüfung 8. 500Hz nach Befreiung 9. LZB-Halt überfahren 10. LZB-Rechnerausfall 11. LZB-Nothalt überfahren 12. Übertragungsausfall in verdeckter Aufnahme
    pub zwangsbremsungsgrund: Option<u16>,
    #[zusi(id = 0x0004)]
    pub zwangsbremsungsgrund_string: Option<String>,
    #[zusi(id = 0x0005)]
    pub lm1000hz: Option<u8>,
    #[zusi(id = 0x0006)]
    pub lm_u: Option<u8>,
    #[zusi(id = 0x0007)]
    pub lm_m: Option<u8>,
    #[zusi(id = 0x0008)]
    pub lm_o: Option<u8>,
    #[zusi(id = 0x0009)]
    /// 1: Hupe 2: Zwangsbremsung
    pub indusi_hupe: Option<u8>,
    #[zusi(id = 0x000A)]
    pub lm500hz: Option<u8>,
    #[zusi(id = 0x000B)]
    pub lm_befehl: Option<u8>,
    #[zusi(id = 0x000C)]
    /// PZB90 0: Normal 1: 1000Hz nach 700m 2: Restriktiv 3: Restriktiv+1000Hz 4: Restriktiv+500Hz 5: Prüfablauf nach LZB-Übertragungsausfall
    pub zusatzinfo_melderbild: Option<u8>,
    #[zusi(id = 0x000D)]
    /// LZB 0: Keine LZB-Führung 1: Normale Fahrt 2: Nothalt 3: LZB-Halt überfahren 4: Rechnerausfall 5: Nachfahrauftrag 6: Funktionsprüfung
    pub lzb: Option<u16>,
    #[zusi(id = 0x000E)]
    pub lzb_ende_verfahren: Option<LzbAuftrag>,
    #[zusi(id = 0x000F)]
    pub lzb_ersatzauftrag: Option<LzbAuftrag>,
    #[zusi(id = 0x0010)]
    pub lzb_falschfahrauftrag: Option<LzbAuftrag>,
    #[zusi(id = 0x0011)]
    pub lzb_vorsichtauftrag: Option<LzbAuftrag>,
    #[zusi(id = 0x0012)]
    pub lzb_fahrt_ueber_halt_befehl: Option<LzbAuftrag>,
    #[zusi(id = 0x0013)]
    pub lzb_uebertragungsausfall: Option<LzbUebertragungsausfall>,
    #[zusi(id = 0x0014)]
    pub lzb_nothalt: Option<LzbNothalt>,
    #[zusi(id = 0x0015)]
    pub lzb_rechnerausfall: Option<LzbRechnerausfall>,
    #[zusi(id = 0x0016)]
    pub lzb_el_auftrag: Option<LzbElAuftrag>,
    #[zusi(id = 0x0017)]
    pub lm_h: Option<u8>,
    #[zusi(id = 0x0018)]
    pub lm_e40: Option<u8>,
    #[zusi(id = 0x0019)]
    pub lm_ende: Option<u8>,
    #[zusi(id = 0x001A)]
    pub lm_b: Option<u8>,
    #[zusi(id = 0x001B)]
    pub lm_ue: Option<u8>,
    #[zusi(id = 0x001C)]
    pub lm_g: Option<u8>,
    #[zusi(id = 0x001D)]
    pub lm_el: Option<u8>,
    #[zusi(id = 0x001E)]
    pub lm_v40: Option<u8>,
    #[zusi(id = 0x001F)]
    pub lm_s: Option<u8>,
    #[zusi(id = 0x0020)]
    pub lm_pruef_stoer: Option<u8>,
    #[zusi(id = 0x0021)]
    /// m/s
    pub sollgeschwindigkeit: Option<f32>,
    #[zusi(id = 0x0022)]
    /// m/s Wert < 0 = Dunkel
    pub zielgeschwindigkeit: Option<f32>,
    #[zusi(id = 0x0023)]
    /// m Wert < 0 = Dunkel
    pub zielweg: Option<f32>,
    #[zusi(id = 0x0024)]
    /// 0: aus 1: an 2: blinkt
    pub lm_gbl: Option<u8>,
    #[zusi(id = 0x0025)]
    /// 0: aus 1: an 2: blinkt
    pub lm_pruef_stoer_bl: Option<u8>,
    #[zusi(id = 0x0026)]
    /// 0: Normal 1: CIR-ELKE aktiv
    pub cir_elke_modus: Option<u8>,
    #[zusi(id = 0x0027)]
    /// 0: Normal 1: Zugdatenanzeige im MFA
    pub anzeigemodus: Option<u8>,
    #[zusi(id = 0x0028)]
    pub lzb_funktionspruefung: Option<LzbFunktionspruefung>,
    #[zusi(id = 0x0029)]
    /// PZB90 S-Bahn
    pub lm_zugart_links: Option<u8>,
    #[zusi(id = 0x002A)]
    /// PZB90 S-Bahn
    pub lm_zugart65: Option<u8>,
    #[zusi(id = 0x002B)]
    /// PZB90 S-Bahn
    pub lm_zugart_rechts: Option<u8>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct LzbAuftrag {
    #[zusi(id = 0x0001)]
    /// 1: eingeleitet 2: quittiert bei Vorsichtauftrag: 3: Fahrt auf Sicht (V40 Melder Dauerlicht)
    pub status: Option<u8>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct LzbUebertragungsausfall {
    #[zusi(id = 0x0001)]
    /// m/s
    pub zielgeschwindigkeit: Option<f32>,
    #[zusi(id = 0x0002)]
    /// 1: eingeleitet 2: Ü Blinkt 3: erste Quittierung erfolt 4: Bedienung für 2. Quittierung gegeben 5: zweite Quittierung erfolgt 6: Ausfall nach verdeckter LZB Aufnahme (CE) 7: dito, Befehl blinkt
    pub status: Option<u16>,
    #[zusi(id = 0x0003)]
    /// nur CIR-ELKE
    pub zielweg: Option<f32>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct LzbNothalt {
    #[zusi(id = 0x0001)]
    /// 1: empfangen 2: überfahren 3: aufgehoben
    pub status: Option<u8>,
    #[zusi(id = 0x0002)]
    /// 1: wird gesendet
    pub wird_gesendet: Option<u8>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct LzbRechnerausfall {
    #[zusi(id = 0x0001)]
    /// 1: alles dunkel 2: Befehlsmelder blinkt nach Neustart 3: Befehlsmelder Dauerlicht nach Quittierung
    pub status: Option<u8>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct LzbElAuftrag {
    #[zusi(id = 0x0001)]
    /// 1: Hauptschalter aus (EL Dauerlicht) 2: Stromabnehmer senken (EL Blinkt)
    pub status: Option<u8>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct LzbFunktionspruefung {
    #[zusi(id = 0x0001)]
    pub alle_melder_blinken: Option<EmptyNode>,
    #[zusi(id = 0x0002)]
    pub anzeige_der_fuehrungsgroessen: Option<EmptyNode>,
    #[zusi(id = 0x0003)]
    pub ban_ueaus: Option<EmptyNode>,
    #[zusi(id = 0x0004)]
    pub zwangsbremsung_aktiv: Option<EmptyNode>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct EtcsEinstellungen {
    #[zusi(id = 0x0001)]
    /// Zusi -> Client
    pub zustand: Option<u8>,
    #[zusi(id = 0x0002)]
    /// Erstes STM = Aktives STM
    pub stm: Vec<EtcsStm>,
    #[zusi(id = 0x0003)]
    pub zugdaten: Option<EtcsZugdaten>,
    #[zusi(id = 0x0004)]
    pub spec: Option<EtcsSpec>,
    #[zusi(id = 0x0005)]
    /// 1: ETCS Abgeschaltet 2: ETCS Eingeschaltet
    pub etcs_stoerschalter: Option<u8>,
    #[zusi(id = 0x0006)]
    /// 1: ETCS Abgeschaltet 2: ETCS Eingeschaltet
    pub etcs_hauptschalter: Option<u8>,
    #[zusi(id = 0x0007)]
    /// 1: Abgesperrt 2: Offen
    pub luftabsperrhahn: Option<u8>,
    #[zusi(id = 0x0008)]
    /// 1: verlegt 2: grundstellung
    pub etcs_quittierschalter: Option<u8>,
    #[zusi(id = 0x0009)]
    /// 1: Override angefordert 2: Grundstellung
    pub override_anforderung: Option<u8>,
    #[zusi(id = 0x000A)]
    /// Nur Client -> Zusi : 1: Startkommando 2: Grundstellung
    pub start: Option<u8>,
    #[zusi(id = 0x000B)]
    /// Client -> Zusi: ETCS-Level (STM, 0, 1, usw.)
    pub level_einstellen_anfordern: Option<u8>,
    #[zusi(id = 0x000C)]
    /// Client -> Zusi
    pub stm_selected_index: Option<u16>,
    #[zusi(id = 0x000D)]
    /// Client -> Zusi
    pub modus_einstellen_anfordern: Option<u16>,
    #[zusi(id = 0x000E)]
    /// Client -> Zusi 1: TAF Quittiert 2: Grundstellung 3: TAF Abgelehnt
    pub taf_modus: Option<u8>,
    #[zusi(id = 0x000F)]
    /// Zusi -> Client 1: Zug wurde neu gestartet bzw. neu uebernommen.
    pub zugneustart: Option<u8>,
    #[zusi(id = 0x0010)]
    /// Client -> Zusi: 1: Zusi soll den Info-Ton 1x abspielen
    pub info_ton_abspielen: Option<u8>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct EtcsStm {
    #[zusi(id = 0x0001)]
    pub stm_index: Option<u16>,
    #[zusi(id = 0x0002)]
    pub stm_name: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct EtcsZugdaten {
    #[zusi(id = 0x0001)]
    /// in %
    pub bremshundertstel: Option<u16>,
    #[zusi(id = 0x0002)]
    pub zugkategorie: Option<u16>,
    #[zusi(id = 0x0003)]
    /// in m
    pub zuglaenge: Option<u16>,
    #[zusi(id = 0x0004)]
    /// in km/h
    pub hoechstgeschwindigkeit: Option<u16>,
    #[zusi(id = 0x0005)]
    /// in kg
    pub achslast: Option<u16>,
    #[zusi(id = 0x0006)]
    pub zugnummer: Option<u16>,
    #[zusi(id = 0x0007)]
    pub tf_nummer: Option<u16>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct EtcsSpec {
    #[zusi(id = 0x0001)]
    /// 1: vermindert 2: nicht vermindert
    pub reibwert: Option<u8>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct EtcsBetriebsdaten {
    #[zusi(id = 0x0001)]
    /**
    0: Undefiniert 1: STM 2: 0 3: 1 4: 2 5: 3

    00: Undefiniert
    01: FS
    02: OS
    03: SR
    04: SH
    05: UN
    06: SL
    07: SB
    08: TR
    09: PT
    10: SF
    11: IS
    12: NP
    13: NL
    14: SE
    15: SN
    16: RV
    17: LS
    18: PS
    **/
    pub aktives_level: Option<u16>,
    #[zusi(id = 0x0002)]
    pub aktiver_modus: Option<u16>,
    #[zusi(id = 0x0003)]
    pub bremsungs_grund: Option<u16>,
    #[zusi(id = 0x0004)]
    pub bremsungs_grund_string: Option<String>,
    #[zusi(id = 0x0005)]
    pub stm_info: Option<EtcsStmInfo>,
    #[zusi(id = 0x0006)]
    pub level_ankuendigung: Option<EtcsLevelAnkuendigung>,
    #[zusi(id = 0x0007)]
    pub modus_ankuendigung: Option<EtcsModusAnkuendigung>,
    #[zusi(id = 0x0008)]
    pub funkstatus: Option<EtcsFunkstatus>,
    #[zusi(id = 0x0009)]
    /// in m/s
    pub zielgeschwindigkeit: Option<f32>,
    #[zusi(id = 0x000A)]
    /// in m, <0 = Dunkel
    pub zielweg: Option<f32>,
    #[zusi(id = 0x000B)]
    /// in m/s
    pub abstand_bremseinsatzpunkt: Option<f32>,
    #[zusi(id = 0x000C)]
    /// in m/s
    pub entlassungsgeschwindigkeit: Option<f32>,
    #[zusi(id = 0x000D)]
    /// in m/s
    pub sollgeschwindigkeit: Option<f32>,
    #[zusi(id = 0x000E)]
    /// in m/s
    pub warngeschwindigkeit: Option<f32>,
    #[zusi(id = 0x000F)]
    /// in m/s
    pub bremsgeschwindigkeit: Option<f32>,
    #[zusi(id = 0x0010)]
    /// in m/s
    pub zwangsbremsgeschwindigkeit: Option<f32>,
    #[zusi(id = 0x0011)]
    /// 0: nein 1: ja
    pub bremskurve_laeuft: Option<u8>,
    #[zusi(id = 0x0012)]
    pub vorschaupunkte: Vec<EtcsVorschaupunkt>,
    #[zusi(id = 0x0013)]
    /// Zusi -> Client
    pub override_aktiv: Option<u8>,
    #[zusi(id = 0x0014)]
    pub notruf_status: Option<u8>,
    #[zusi(id = 0x0015)]
    pub betriebszwangsbremsung: Option<u8>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct EtcsStmInfo {
    #[zusi(id = 0x0001)]
    /// Index des aktiven STM-System, von 1 beginnend gemäß Reihenfolge in der ftd-Datei
    pub stm_index: Option<u16>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct EtcsLevelAnkuendigung {
    #[zusi(id = 0x0001)]
    pub neues_level: Option<u16>,
    #[zusi(id = 0x0002)]
    pub quittierung: Option<u8>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct EtcsModusAnkuendigung {
    #[zusi(id = 0x0001)]
    pub neuer_modus: Option<u16>,
    #[zusi(id = 0x0002)]
    pub quittierung: Option<u8>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct EtcsFunkstatus {
    #[zusi(id = 0x0001)]
    pub zustand: Option<u8>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct EtcsVorschaupunkt {
    #[zusi(id = 0x0001)]
    /// 1: Strecke 3: Hauptsignal 9: Rangiersignal 14: ETCS
    pub herkunft: Option<u16>,
    #[zusi(id = 0x0002)]
    /// in m/s, -1 = ETCS Ende
    pub geschwindigkeit: Option<f32>,
    #[zusi(id = 0x0003)]
    /// in m
    pub abstand: Option<f32>,
    #[zusi(id = 0x0004)]
    /// in m
    pub hoehenwert: Option<f32>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct ZubEinstellungen {
    #[zusi(id = 0x0001)]
    pub brh_wert: Option<u16>,
    #[zusi(id = 0x0002)]
    pub zuglaenge: Option<u16>,
    #[zusi(id = 0x0003)]
    /// in km/h
    pub vmax: Option<u16>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct ZubBetriebsdaten {
    #[zusi(id = 0x0001)]
    pub lm_gnt: Option<u8>,
    #[zusi(id = 0x0002)]
    pub lm_gnt_ue: Option<u8>,
    #[zusi(id = 0x0003)]
    pub lm_gnt_g: Option<u8>,
    #[zusi(id = 0x0004)]
    pub lm_gnt_s: Option<u8>,
    #[zusi(id = 0x0005)]
    pub lm_gnt_gst: Option<u8>,
    #[zusi(id = 0x0006)]
    pub lm_gnt_stoer: Option<u8>,
    #[zusi(id = 0x0007)]
    pub status_lm_gnt_ue: Option<u8>,
    #[zusi(id = 0x0008)]
    pub status_lm_gnt_g: Option<u8>,
    #[zusi(id = 0x0009)]
    pub status_lm_gnt_s: Option<u8>,
    #[zusi(id = 0x000A)]
    pub zwangsbremsung: Option<u16>,
    #[zusi(id = 0x000B)]
    pub betriebsbremsung_aktiv: Option<u8>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
/// 5.3.3.3.5
pub struct StatusTuersystem {
    #[zusi(id = 0x0001)]
    pub bauart: Option<String>,
    #[zusi(id = 0x0002)]
    /// 0: zu 1: öffnend 2: offen 3: Fahrgastwechsel abgeschlossen 4: schließend 5: gestört 6: blockiert
    pub status_links: Option<u8>,
    #[zusi(id = 0x0003)]
    /// 0: zu 1: öffnend 2: offen 3: Fahrgastwechsel abgeschlossen 4: schließend 5: gestört 6: blockiert
    pub status_rechts: Option<u8>,
    #[zusi(id = 0x0004)]
    /// 1: an
    pub traktionssperre_aktiv: Option<u8>,
    #[zusi(id = 0x0005)]
    /// 0: zu 1: links 2: rechts 3: beide
    pub seitenwahlschalter: Option<u8>,
    #[zusi(id = 0x0006)]
    pub lm_links: Option<u8>,
    #[zusi(id = 0x0007)]
    pub lm_rechts: Option<u8>,
    #[zusi(id = 0x0008)]
    pub status_lm_links: Option<u8>,
    #[zusi(id = 0x0009)]
    pub status_lm_rechts: Option<u8>,
    #[zusi(id = 0x000A)]
    pub lm_zwangsschliessen: Option<u8>,
    #[zusi(id = 0x000B)]
    pub status_lm_zwangsschliessen: Option<u8>,
    #[zusi(id = 0x000C)]
    pub lm_links_und_rechts: Option<u8>,
    #[zusi(id = 0x000D)]
    pub status_lm_links_und_rechts: Option<u8>,
    #[zusi(id = 0x000E)]
    pub zentrales_oeffnen_links: Option<u8>,
    #[zusi(id = 0x000F)]
    pub zentrales_oeffnen_rechts: Option<u8>,
    #[zusi(id = 0x0010)]
    /// 0: Aus 1: Dauerlicht 2: Blinkend
    pub status_zentrales_oeffnen_links: Option<u8>,
    #[zusi(id = 0x0011)]
    /// 0: Aus 1: Dauerlicht 2: Blinkend
    pub status_zentrales_oeffnen_rechts: Option<u8>,
    #[zusi(id = 0x0012)]
    /// 1: an
    pub lm_gruenschleife: Option<u8>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
/// 5.3.3.3.6
pub struct StatusFahrzeug {
    #[zusi(id = 0x0001)]
    /// 0: nichts 1: niedriger HLL druck 2: dynamische Bremse 3: traktionssperre
    pub grund_nullstellungszwang: Option<u16>,
    #[zusi(id = 0x0002)]
    /// 0: nichts 1: Federspeichenbremse 2: Türsystem 3: Bremsprobe läuft
    pub grund_traktionssperre: Option<u16>,
    #[zusi(id = 0x0003)]
    /// 1: deaktivert 2: normalzustand 3: gestört
    pub status_fahrschalter: Option<u8>,
    #[zusi(id = 0x0004)]
    /// 1: deaktivert 2: normalzustand
    pub status_dynamische_bremse: Option<u8>,
    #[zusi(id = 0x0005)]
    pub status_sander: Option<u8>,
    #[zusi(id = 0x0006)]
    pub status_bremsprobe: Option<u8>,
    #[zusi(id = 0x0007)]
    pub stellung_richtungsschalter: Option<u8>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
/// 5.3.3.3.7
pub struct StatusZugverband {
    #[zusi(id = 0x0001)]
    pub status_fahrzeug: Vec<Einzelfahrzeug>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct Einzelfahrzeug {
    #[zusi(id = 0x0001)]
    pub dateiname: Option<String>,
    #[zusi(id = 0x0002)]
    pub beschreibung: Option<String>,
    #[zusi(id = 0x0003)]
    pub bremsstellung: Option<u16>, // 0: keine/undefiniert 1: G 2: P 3 : P+Mg 4: R 5: R+Mg
    #[zusi(id = 0x0004)]
    pub zugbeeinflussungssystem: Vec<EfzZugbeeinflussungssystem>,
    #[zusi(id = 0x0005)]
    pub vmax: Option<f32>, // m/s
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct EfzZugbeeinflussungssystem {
    #[zusi(id = 0x0001)]
    pub bezeichnung: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
/// 5.3.3.3.8 - Status Weichen
pub struct StatusWeichen {
    #[zusi(id = 0x0001)]
    pub weichen: Vec<Weiche>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct Weiche {
    #[zusi(id = 0x0001)]
    pub bezeichnung: Option<String>,
    #[zusi(id = 0x0002)]
    pub bauart: Option<i32>,
    #[zusi(id = 0x0003)]
    pub grundstellung: Option<i32>,
    #[zusi(id = 0x0004)]
    pub lage: Option<u8>,
    #[zusi(id = 0x0005)]
    pub fahrtrichtung: Option<u8>,
    #[zusi(id = 0x0006)]
    pub umlaufmodus_stumpfbefahrung: Option<u8>,
}

// 5.3.3.4 Befehl 00 0B - DATA_OPERATION (Zusi → Client)

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
/// 5.3.3.5 Befehl 00 0C - DATA_PROG (Zusi → Client)
pub struct DataProg {
    #[zusi(id = 0x0001)]
    pub zugdatei: Option<String>,
    #[zusi(id = 0x0002)]
    pub zugnummer: Option<String>,
    #[zusi(id = 0x0003)]
    pub ladepause: Option<u8>, // 0: Ende Ladepause
    #[zusi(id = 0x0004)]
    pub buchfahrplandatei: Option<String>, // serialisertes xml
}

// 5.3.3.6 Befehl 01 0A - INPUT (Client → Zusi)

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
/// 5.3.3.7 Befehl 01 0B - CONTROL (Client → Zusi)
pub struct Control {
    #[zusi(id = 0x0001)]
    pub pause: Option<ShortNode>,
    #[zusi(id = 0x0002)]
    pub restart: Option<ControlFilename>,
    #[zusi(id = 0x0003)]
    pub start: Option<ControlFilename>,
    #[zusi(id = 0x0004)]
    pub ende: Option<EmptyNode>,
    #[zusi(id = 0x0005)]
    pub fpl_neustart: Option<EmptyNode>,
    #[zusi(id = 0x0006)]
    pub zug_auswaehlen: Option<Train>, // nach Fahrplan neu starten
    #[zusi(id = 0x0007)]
    pub zeitsprung: Option<ShortNode>,
    #[zusi(id = 0x0008)]
    pub zeitraffer: Option<ShortNode>,
    #[zusi(id = 0x0009)]
    pub nebel: Option<SingleNode>,
    #[zusi(id = 0x000A)]
    pub helligkeit: Option<SingleNode>,
    #[zusi(id = 0x000B)]
    pub reibwert: Option<SingleNode>,
    #[zusi(id = 0x000C)]
    pub autopilot: Option<ShortNode>,
}

// 5.3.3.8 Befehl 01 0C - GRAPHIC (Client → Zusi)

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct ShortNode {
    #[zusi(id = 0x0001)]
    pub control: Option<i16>, // -1: umschalten, 0: aus, 1: ein
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct SingleNode {
    #[zusi(id = 0x0001)]
    pub control: Option<f32>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct ControlFilename {
    #[zusi(id = 0x0001)]
    /// Dateiname des Zuges relativ zum Zusi-Verzeichnis. Wird ein Leerstring übermittelt, startet der zuletzt gefahrene Zug.
    pub dateiname: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct EmptyNode {}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct Train {
    #[zusi(id = 0x0001)]
    pub zugnummer: Option<String>,
}
