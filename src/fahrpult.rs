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
    pub geschwindigkeit: Option<f32>, // Single: m/s
    #[zusi(id = 0x0002)]
    pub druck_hauptluftleitung: Option<f32>, // Single: bar
    #[zusi(id = 0x0003)]
    pub druck_bremszylinder: Option<f32>, // Single: bar
    #[zusi(id = 0x0004)]
    pub druck_hauptluftbehaelter: Option<f32>, // Single: bar
    #[zusi(id = 0x0005)]
    pub luftpresser_laeuft: Option<f32>, // Single: aus/an
    #[zusi(id = 0x0006)]
    pub luftstrom_fvb: Option<f32>, // Single: -1..0..+1
    #[zusi(id = 0x0007)]
    pub luftstrom_zbv: Option<f32>, // Single: -1..0..+1
    #[zusi(id = 0x0008)]
    pub luefter_an: Option<f32>, // Single: aus/an
    #[zusi(id = 0x0009)]
    pub zugkraft_gesamt: Option<f32>, // Single: N
    #[zusi(id = 0x000A)]
    pub zugkraft_pro_achse: Option<f32>, // Single: N
    #[zusi(id = 0x000B)]
    pub zugkraft_soll_gesamt: Option<f32>, // Single: N
    #[zusi(id = 0x000C)]
    pub zugkraft_soll_pro_achse: Option<f32>, // Single: N
    #[zusi(id = 0x000D)]
    pub oberstrom: Option<f32>, // Single: A
    #[zusi(id = 0x000E)]
    pub fahrleitungsspannung: Option<f32>, // Single: V
    #[zusi(id = 0x000F)]
    pub motordrehzahl: Option<f32>, // Single: 1/min
    #[zusi(id = 0x0010)]
    pub uhrzeit_stunde: Option<f32>, // Single: Stunde (Zeigerposition Analoguhren)
    #[zusi(id = 0x0011)]
    pub uhrzeit_minute: Option<f32>, // Single: Minute (Zeigerposition Analoguhren)
    #[zusi(id = 0x0012)]
    pub uhrzeit_sekunde: Option<f32>, // Single: Sekunde (Zeigerposition Analoguhren)
    #[zusi(id = 0x0013)]
    pub hauptschalter: Option<f32>, // Single: aus/an
    #[zusi(id = 0x0014)]
    pub trennschuetz: Option<f32>, // Single: aus/an
    #[zusi(id = 0x0015)]
    pub fahrstufe: Option<f32>, // Single: 1
    //#[zusi(id = 0x0016)]
    //d3DFenster                                       : Option<f32>, // Single: Nicht sinnvoll zu verwenden
    #[zusi(id = 0x0017)]
    pub afb_sollgeschwindigkeit: Option<f32>, // Single: m/s
    #[zusi(id = 0x0018)]
    pub druck_hilfsluftbehaelter: Option<f32>, // Single: bar
    #[zusi(id = 0x0019)]
    pub zurueckgelegter_gesamtweg: Option<f32>, // Single: m
    #[zusi(id = 0x001A)]
    pub lm_getriebe: Option<f32>, // Single: aus/an
    #[zusi(id = 0x001B)]
    pub lm_schleudern: Option<f32>, // Single: aus/an
    #[zusi(id = 0x001C)]
    pub lm_gleiten: Option<f32>, // Single: aus/an
    #[zusi(id = 0x001D)]
    pub lm_mg_bremse: Option<f32>, // Single: aus/an
    #[zusi(id = 0x001E)]
    pub lm_hbremse: Option<f32>, // Single: aus/an
    #[zusi(id = 0x001F)]
    pub lm_rbremse: Option<f32>, // Single: aus/an
    #[zusi(id = 0x0020)]
    pub lm_hochabbremsung: Option<f32>, // Single: aus/an
    #[zusi(id = 0x0021)]
    pub lm_schnellbremsung: Option<f32>, // Single: aus/an
    #[zusi(id = 0x0022)]
    pub status_notbremssystem: Option<StatusNotbremssystem>,
    #[zusi(id = 0x0023)]
    pub lm_uhrzeit_digital: Option<f32>, // Single: 0...1 (0:00 bis 24:00 Uhr)
    #[zusi(id = 0x0024)]
    pub lm_drehzahlverstellung: Option<f32>, // Single: aus/an
    #[zusi(id = 0x0025)]
    pub lm_fahrtrichtung_vor: Option<f32>, // Single: aus/an
    #[zusi(id = 0x0026)]
    pub lm_fahrtrichtung_zurueck: Option<f32>, // Single: aus/an
    #[zusi(id = 0x0027)]
    pub lm_fahrtrichtung_m: Option<f32>, // Single: aus/an
    // #[zusi(id = 0x0028)]
    // hintergrundbild: Option<f32>, // Single: Nicht sinnvoll zu verwenden
    #[zusi(id = 0x0029)]
    pub motordrehmoment: Option<f32>, // Single: Nm
    #[zusi(id = 0x002A)]
    pub motorlast_normiert: Option<f32>, // Single: 1 (0...1)
    #[zusi(id = 0x002B)]
    pub tunnel: Option<f32>, // Single: aus/an
    #[zusi(id = 0x002C)]
    pub schienenstoss_weiche: Option<f32>, // Single: aus/an
    #[zusi(id = 0x002D)]
    pub stahlbruecke: Option<f32>, // Single: aus/an
    #[zusi(id = 0x002E)]
    pub steinbruecke: Option<f32>, // Single: aus/an
    #[zusi(id = 0x002F)]
    pub xkoordinate: Option<f32>, // Single: m (Bez. Strecken-UTM-Punkt)
    #[zusi(id = 0x0030)]
    pub ykoordinate: Option<f32>, // Single: m (Bez. Strecken-UTM-Punkt)
    #[zusi(id = 0x0031)]
    pub zkoordinate: Option<f32>, // Single: m
    #[zusi(id = 0x0032)]
    pub utm_referenzpunkt_xkm: Option<f32>, // Single: km
    #[zusi(id = 0x0033)]
    pub utmr_eferenzpunkt_ykm: Option<f32>, // Single: km
    #[zusi(id = 0x0034)]
    pub utm_zone: Option<f32>, // Single
    #[zusi(id = 0x0035)]
    pub utm_zone2: Option<f32>, // Single
    #[zusi(id = 0x0036)]
    pub afb_an: Option<f32>, // Single: aus/an
    #[zusi(id = 0x0037)]
    pub individuell01: Option<f32>, // Single
    #[zusi(id = 0x0038)]
    pub individuell02: Option<f32>, // Single
    #[zusi(id = 0x0039)]
    pub individuell03: Option<f32>, // Single
    #[zusi(id = 0x003A)]
    pub individuell04: Option<f32>, // Single
    #[zusi(id = 0x003B)]
    pub individuell05: Option<f32>, // Single
    #[zusi(id = 0x003C)]
    pub individuell06: Option<f32>, // Single
    #[zusi(id = 0x003D)]
    pub individuell07: Option<f32>, // Single
    #[zusi(id = 0x003E)]
    pub individuell08: Option<f32>, // Single
    #[zusi(id = 0x003F)]
    pub individuell09: Option<f32>, // Single
    #[zusi(id = 0x0040)]
    pub individuell10: Option<f32>, // Single
    #[zusi(id = 0x0041)]
    pub individuell11: Option<f32>, // Single
    #[zusi(id = 0x0042)]
    pub individuell12: Option<f32>, // Single
    #[zusi(id = 0x0043)]
    pub individuell13: Option<f32>, // Single
    #[zusi(id = 0x0044)]
    pub individuell14: Option<f32>, // Single
    #[zusi(id = 0x0045)]
    pub individuell15: Option<f32>, // Single
    #[zusi(id = 0x0046)]
    pub individuell16: Option<f32>, // Single
    #[zusi(id = 0x0047)]
    pub individuell17: Option<f32>, // Single
    #[zusi(id = 0x0048)]
    pub individuell18: Option<f32>, // Single
    #[zusi(id = 0x0049)]
    pub individuell19: Option<f32>, // Single
    #[zusi(id = 0x004A)]
    pub individuell20: Option<f32>, // Single
    #[zusi(id = 0x004B)]
    pub datum: Option<f32>, // Single (Tage mit 0 = 30.12.1899)
    #[zusi(id = 0x004C)]
    pub gleiskruemmung: Option<f32>, // Single 1000/m
    #[zusi(id = 0x004D)]
    pub streckenhoechstgeschwindigkeit: Option<f32>, // Single: m/s
    #[zusi(id = 0x004E)]
    pub zugkraftvorschlag_autopilot: Option<f32>, // Single: N
    #[zusi(id = 0x004F)]
    pub beschleunigung_x: Option<f32>, // Single: m/s^2
    #[zusi(id = 0x0050)]
    pub beschleunigung_y: Option<f32>, // Single: m/s^2
    #[zusi(id = 0x0051)]
    pub beschleunigung_z: Option<f32>, // Single: m/s^2
    #[zusi(id = 0x0052)]
    pub drehbeschleunigung_x_achse: Option<f32>, // Single: rad/s^2
    #[zusi(id = 0x0053)]
    pub drehbeschleunigung_y_achse: Option<f32>, // Single: rad/s^2
    #[zusi(id = 0x0054)]
    pub drehbeschleunigung_z_achse: Option<f32>, // Single: rad/s^2
    #[zusi(id = 0x0055)]
    pub stromabnehmer: Option<f32>, // Single: 2x4 bit (4bit: 1 fuer SA=oben; 4 bit: 1 fuer SA hebt sich gerade)
    #[zusi(id = 0x0056)]
    pub lm_federspeicherbremse: Option<f32>, // Single: aus/an
    #[zusi(id = 0x0057)]
    pub zustand_federspeicherbremse: Option<f32>, // Single -1, 0, 1, 2 (nicht vorhanden, aus, an, blinkend)
    #[zusi(id = 0x0058)]
    pub steuerwagen_lm_getriebe: Option<f32>, // Single: aus/an
    #[zusi(id = 0x0059)]
    pub steuerwagen_lm_schleudern: Option<f32>, // Single: aus/an
    #[zusi(id = 0x005A)]
    pub steuerwagen_lm_gleiten: Option<f32>, // Single: aus/an
    #[zusi(id = 0x005B)]
    pub steuerwagen_lm_hbremse: Option<f32>, // Single: aus/an
    #[zusi(id = 0x005C)]
    pub steuerwagen_lm_rbremse: Option<f32>, // Single: aus/an
    #[zusi(id = 0x005D)]
    pub steuerwagen_lm_drehzahlverstellung: Option<f32>, // Single: aus/an
    #[zusi(id = 0x005E)]
    pub druck_zweitbehaelter: Option<f32>, // Single: bar
    #[zusi(id = 0x005F)]
    pub geschwindigkeit_absolut: Option<f32>, // Single: m/s
    #[zusi(id = 0x0060)]
    pub zug_ist_entgleist: Option<f32>, // Single: aus/an
    #[zusi(id = 0x0061)]
    pub kilometrierung_zugspitze: Option<f32>, // Single: km
    #[zusi(id = 0x0062)]
    pub motorstrom: Option<f32>, // Single: A
    #[zusi(id = 0x0063)]
    pub motorspannung: Option<f32>, // Single: V
    #[zusi(id = 0x0064)]
    pub status_sifa: Option<StatusSifa>,
    #[zusi(id = 0x0065)]
    pub status_zugsicherung: Option<StatusZugbeeinflussung>,
    #[zusi(id = 0x0066)]
    pub status_tuersystem: Option<StatusTuersystem>,
    #[zusi(id = 0x0067)]
    pub individuell21: Option<f32>, // Single
    #[zusi(id = 0x0068)]
    pub individuell22: Option<f32>, // Single
    #[zusi(id = 0x0069)]
    pub individuell23: Option<f32>, // Single
    #[zusi(id = 0x006A)]
    pub individuell24: Option<f32>, // Single
    #[zusi(id = 0x006B)]
    pub individuell25: Option<f32>, // Single
    #[zusi(id = 0x006C)]
    pub individuell26: Option<f32>, // Single
    #[zusi(id = 0x006D)]
    pub individuell27: Option<f32>, // Single
    #[zusi(id = 0x006E)]
    pub individuell28: Option<f32>, // Single
    #[zusi(id = 0x006F)]
    pub individuell29: Option<f32>, // Single
    #[zusi(id = 0x0070)]
    pub individuell30: Option<f32>, // Single
    #[zusi(id = 0x0071)]
    pub individuell31: Option<f32>, // Single
    #[zusi(id = 0x0072)]
    pub individuell32: Option<f32>, // Single
    #[zusi(id = 0x0073)]
    pub individuell33: Option<f32>, // Single
    #[zusi(id = 0x0074)]
    pub individuell34: Option<f32>, // Single
    #[zusi(id = 0x0075)]
    pub individuell35: Option<f32>, // Single
    #[zusi(id = 0x0076)]
    pub individuell36: Option<f32>, // Single
    #[zusi(id = 0x0077)]
    pub individuell37: Option<f32>, // Single
    #[zusi(id = 0x0078)]
    pub individuell38: Option<f32>, // Single
    #[zusi(id = 0x0079)]
    pub individuell39: Option<f32>, // Single
    #[zusi(id = 0x007A)]
    pub individuell40: Option<f32>, // Single
    #[zusi(id = 0x007B)]
    pub steuerwagen_luefter_an: Option<f32>, // Single: aus/an
    #[zusi(id = 0x007C)]
    pub steuerwagen_zugkraft_gesamt: Option<f32>, // Single: N
    #[zusi(id = 0x007D)]
    pub steuerwagen_zugraft_pro_achse: Option<f32>, // Single: N
    #[zusi(id = 0x007E)]
    pub steuerwagen_zugkraft_soll_gesamt: Option<f32>, // Single: N
    #[zusi(id = 0x007F)]
    pub steuerwagen_zugraft_soll_pro_achse: Option<f32>, // Single: N
    #[zusi(id = 0x0080)]
    pub steuerwagen_oberstrom: Option<f32>, // Single: N
    #[zusi(id = 0x0081)]
    pub steuerwagen_fahrleitungsspannung: Option<f32>, // Single V
    #[zusi(id = 0x0082)]
    pub steuerwagen_motordrehzahl: Option<f32>, // Single 1/min
    #[zusi(id = 0x0083)]
    pub steuerwagen_hauptschalter: Option<f32>, // Single aus/an
    #[zusi(id = 0x0084)]
    pub steuerwagen_trennschuetz: Option<f32>, // Single: aus/an
    #[zusi(id = 0x0085)]
    pub steuerwagen_fahrstufe: Option<f32>, // Single: 1
    #[zusi(id = 0x0086)]
    pub steuerwagen_motordrehmoment: Option<f32>, // Single: Nm
    #[zusi(id = 0x0087)]
    pub steuerwagen_motorlast_normiert: Option<f32>, // Single: 1 (0...1)
    #[zusi(id = 0x0088)]
    pub steuerwagen_stromabnehmer: Option<f32>, // Single
    #[zusi(id = 0x0089)]
    pub steuerwagen_motorstrom: Option<f32>, // Single: A
    #[zusi(id = 0x008A)]
    pub steuerwagen_motorspannung: Option<f32>, // Single: V
    #[zusi(id = 0x008B)]
    pub geschwindigkeit_absolut_inkl_schleudern: Option<f32>, // Single: m/s
    #[zusi(id = 0x008C)]
    pub batteriehauptschalter_aus: Option<f32>, // Single: aus/an
    #[zusi(id = 0x008D)]
    pub status_fahrzeug: Option<StatusFahrzeug>,
    #[zusi(id = 0x008E)]
    pub status_zugverband: Option<StatusZugverband>,
    #[zusi(id = 0x008F)]
    pub bremsprobenfunktion: Option<f32>, // Single: 0 (aus, >0 aktiv)
    #[zusi(id = 0x0090)]
    pub zug_und_brems_gesamtkraft_soll_normiert: Option<f32>, // Single: 1 ((0...1) normiert auf aktuelle Fmax
    #[zusi(id = 0x0091)]
    pub steuerwagen_zug_und_brems_gesamtkraft_soll_normiert: Option<f32>, // Single: 1 ((0...1) normiert auf aktuelle Fmax
    #[zusi(id = 0x0092)]
    pub status_weichen: Option<StatusWeichen>,
    #[zusi(id = 0x0093)]
    pub zug_und_brems_gesamtkraft_absolut_normiert: Option<f32>, // Single: 1 ((0...1) normiert auf aktuelle Fmax
    #[zusi(id = 0x0094)]
    pub steuerwagen_zug_und_brems_gesamtkraft_absolut_normiert: Option<f32>, // Single: 1 ((0...1) normiert auf aktuelle Fmax
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
/// 5.3.3.3.2
pub struct StatusNotbremssystem {
    #[zusi(id = 0x0001)]
    pub bauart: Option<String>,
    #[zusi(id = 0x0002)]
    pub status_notbremssystem: Option<u8>, // 0: NBÜ aus 1: NBÜ bereit 2: Notbremse gezogen 3: Notbremse wirkt (NBÜ bereit) 4: NBÜ durch Lokführer aktiviert 5 Notbremse wirkt (NBÜ aus)
    #[zusi(id = 0x0003)]
    pub lm_system_bereit: Option<u8>, // 1: an
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
    pub zugart: Option<u8>, // Zugart:1: Zugart muss noch bestimmt werden 2: U 3: M 4: O	5: S-Bahn-Modus
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
    pub hauptschalter: Option<u8>, // Hauptschalter   1: Indusi ausgeschaltet 2: Indusi eingeschaltet
    #[zusi(id = 0x0008)]
    pub pzb_stoerschalter: Option<u8>, // Störschalter    1: Indusi abgeschaltet  2: Indusi eingeschaltet
    #[zusi(id = 0x0009)]
    pub lzb_stoerschalter: Option<u8>, // LZB
    #[zusi(id = 0x000A)]
    pub luftabsperrhahn: Option<u8>, // Luftabsperrhahn 1: abgesperrt           2: offen
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct Zugdaten {
    #[zusi(id = 0x0001)]
    pub bremshundertstel: Option<u16>,
    #[zusi(id = 0x0002)]
    pub bremsart: Option<u16>,
    #[zusi(id = 0x0003)]
    pub zuglaenge: Option<u16>, // LZB
    #[zusi(id = 0x0004)]
    pub vmax: Option<u16>, // LZB in km/h
    #[zusi(id = 0x0005)]
    pub zugart: Option<u8>,
    #[zusi(id = 0x0006)]
    pub modus: Option<u8>, // Nur relevant für AktiveZugdaten. 5: Ersatzzugdaten 6: Normalbetrieb
    #[zusi(id = 0x000B)]
    pub klartextmeldungen: Option<u8>, // 0: Keine Klartextmeldungen möglich 1: Keine Klartextmeldungen möglich aber nicht aktiv 2: Klartextmeldungen aktiv 3: nur Klartextmeldungen möglich
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct IndusiZustand {
    #[zusi(id = 0x0002)]
    zugsicherung: Option<u16>, // 1: Ausgeschaltet 2: abgeschaltet/gestört (1000Hz blinkt) 3: Hauptluftleitung unter 2,2 bar (1000Hz blinkt) 4: Aufforderung zur Zugdateneingabe 5: Normalbetrieb 6: Funktionsprüfung
    #[zusi(id = 0x0003)]
    zwangsbremsungsgrund: Option<u16>, // 0: keine Zwangsbremsung, sonst Zwansbremsung aktiv wegen: 1. Wachsam 2. 1000Hz 3. 500Hz 4. 2000Hz 5. Kein Halt nach Befreiung aus Zwangsbremsung 6. Fahrzeug-v-Max überschritten 7. Funktionsprüfung 8. 500Hz nach Befreiung 9. LZB-Halt überfahren 10. LZB-Rechnerausfall 11. LZB-Nothalt überfahren 12. Übertragungsausfall in verdeckter Aufnahme
    #[zusi(id = 0x0004)]
    zwangsbremsungsgrund_string: Option<String>,
    #[zusi(id = 0x0005)]
    lm1000hz: Option<u8>,
    #[zusi(id = 0x0006)]
    lm_u: Option<u8>,
    #[zusi(id = 0x0007)]
    lm_m: Option<u8>,
    #[zusi(id = 0x0008)]
    lm_o: Option<u8>,
    #[zusi(id = 0x0009)]
    indusi_hupe: Option<u8>, // 1: Hupe 2: Zwangsbremsung
    #[zusi(id = 0x000A)]
    lm500hz: Option<u8>,
    #[zusi(id = 0x000B)]
    lm_befehl: Option<u8>,
    #[zusi(id = 0x000C)]
    zusatzinfo_melderbild: Option<u8>, // PZB90 0: Normal 1: 1000Hz nach 700m 2: Restriktiv 3: Restriktiv+1000Hz 4: Restriktiv+500Hz 5: Prüfablauf nach LZB-Übertragungsausfall
    #[zusi(id = 0x000D)]
    lzb: Option<u16>, // LZB 0: Keine LZB-Führung 1: Normale Fahrt 2: Nothalt 3: LZB-Halt überfahren 4: Rechnerausfall 5: Nachfahrauftrag 6: Funktionsprüfung
    #[zusi(id = 0x000E)]
    lzb_ende_verfahren: Option<LzbAuftrag>,
    #[zusi(id = 0x000F)]
    lzb_ersatzauftrag: Option<LzbAuftrag>,
    #[zusi(id = 0x0010)]
    lzb_falschfahrauftrag: Option<LzbAuftrag>,
    #[zusi(id = 0x0011)]
    lzb_vorsichtauftrag: Option<LzbAuftrag>,
    #[zusi(id = 0x0012)]
    lzb_fahrt_ueber_halt_befehl: Option<LzbAuftrag>,
    #[zusi(id = 0x0013)]
    lzb_uebertragungsausfall: Option<LzbUebertragungsausfall>,
    #[zusi(id = 0x0014)]
    lzb_nothalt: Option<LzbNothalt>,
    #[zusi(id = 0x0015)]
    lzb_rechnerausfall: Option<LzbRechnerausfall>,
    #[zusi(id = 0x0016)]
    lzb_el_auftrag: Option<LzbElAuftrag>,
    #[zusi(id = 0x0017)]
    lm_h: Option<u8>,
    #[zusi(id = 0x0018)]
    lm_e40: Option<u8>,
    #[zusi(id = 0x0019)]
    lm_ende: Option<u8>,
    #[zusi(id = 0x001A)]
    lm_b: Option<u8>,
    #[zusi(id = 0x001B)]
    lm_ue: Option<u8>,
    #[zusi(id = 0x001C)]
    lm_g: Option<u8>,
    #[zusi(id = 0x001D)]
    lm_el: Option<u8>,
    #[zusi(id = 0x001E)]
    lm_v40: Option<u8>,
    #[zusi(id = 0x001F)]
    lm_s: Option<u8>,
    #[zusi(id = 0x0020)]
    lm_pruef_stoer: Option<u8>,
    #[zusi(id = 0x0021)]
    sollgeschwindigkeit: Option<f32>, // m/s
    #[zusi(id = 0x0022)]
    zielgeschwindigkeit: Option<f32>, // m/s Wert < 0 = Dunkel
    #[zusi(id = 0x0023)]
    zielweg: Option<f32>, // m Wert < 0 = Dunkel
    #[zusi(id = 0x0024)]
    lm_gbl: Option<u8>, // 0: aus 1: an 2: blinkt
    #[zusi(id = 0x0025)]
    lm_pruef_stoer_bl: Option<u8>, // 0: aus 1: an 2: blinkt
    #[zusi(id = 0x0026)]
    cir_elke_modus: Option<u8>, // 0: Normal 1: CIR-ELKE aktiv
    #[zusi(id = 0x0027)]
    anzeigemodus: Option<u8>, // 0: Normal 1: Zugdatenanzeige im MFA
    #[zusi(id = 0x0028)]
    lzb_funktionspruefung: Option<LzbFunktionspruefung>,
    #[zusi(id = 0x0029)]
    lm_zugart_links: Option<u8>, // PZB90 S-Bahn
    #[zusi(id = 0x002A)]
    lm_zugart65: Option<u8>, // PZB90 S-Bahn
    #[zusi(id = 0x002B)]
    lm_zugart_rechts: Option<u8>, // PZB90 S-Bahn
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct LzbAuftrag {
    #[zusi(id = 0x0001)]
    status: Option<u8>, // 1: eingeleitet 2: quittiert bei Vorsichtauftrag: 3: Fahrt auf Sicht (V40 Melder Dauerlicht)
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct LzbUebertragungsausfall {
    #[zusi(id = 0x0001)]
    zielgeschwindigkeit: Option<f32>, // m/s
    #[zusi(id = 0x0002)]
    status: Option<u16>, // 1: eingeleitet 2: Ü Blinkt 3: erste Quittierung erfolt 4: Bedienung für 2. Quittierung gegeben 5: zweite Quittierung erfolgt 6: Ausfall nach verdeckter LZB Aufnahme (CE) 7: dito, Befehl blinkt
    #[zusi(id = 0x0003)]
    zielweg: Option<f32>, // nur CIR-ELKE
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct LzbNothalt {
    #[zusi(id = 0x0001)]
    status: Option<u8>, // 1: empfangen 2: überfahren 3: aufgehoben
    #[zusi(id = 0x0002)]
    wird_gesendet: Option<u8>, // 1: wird gesendet
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct LzbRechnerausfall {
    #[zusi(id = 0x0001)]
    status: Option<u8>, // 1: alles dunkel 2: Befehlsmelder blinkt nach Neustart 3: Befehlsmelder Dauerlicht nach Quittierung
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct LzbElAuftrag {
    #[zusi(id = 0x0001)]
    status: Option<u8>, // 1: Hauptschalter aus (EL Dauerlicht) 2: Stromabnehmer senken (EL Blinkt)
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct LzbFunktionspruefung {
    #[zusi(id = 0x0001)]
    alle_melder_blinken: Option<EmptyNode>,
    #[zusi(id = 0x0002)]
    anzeige_der_fuehrungsgroessen: Option<EmptyNode>,
    #[zusi(id = 0x0003)]
    ban_ueaus: Option<EmptyNode>,
    #[zusi(id = 0x0004)]
    zwangsbremsung_aktiv: Option<EmptyNode>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct EtcsEinstellungen {
    #[zusi(id = 0x0001)]
    zustand: Option<u8>, // Zusi -> Client
    #[zusi(id = 0x0002)]
    stm: Vec<EtcsStm>, // Erstes STM = Aktives STM
    #[zusi(id = 0x0003)]
    zugdaten: Option<EtcsZugdaten>,
    #[zusi(id = 0x0004)]
    spec: Option<EtcsSpec>,
    #[zusi(id = 0x0005)]
    etcs_stoerschalter: Option<u8>, // 1: ETCS Abgeschaltet 2: ETCS Eingeschaltet
    #[zusi(id = 0x0006)]
    etcs_hauptschalter: Option<u8>, // 1: ETCS Abgeschaltet 2: ETCS Eingeschaltet
    #[zusi(id = 0x0007)]
    luftabsperrhahn: Option<u8>, // 1: Abgesperrt 2: Offen
    #[zusi(id = 0x0008)]
    etcs_quittierschalter: Option<u8>, // 1: verlegt 2: grundstellung
    #[zusi(id = 0x0009)]
    override_anforderung: Option<u8>, // 1: Override angefordert 2: Grundstellung
    #[zusi(id = 0x000A)]
    start: Option<u8>, // Nur Client -> Zusi : 1: Startkommando 2: Grundstellung
    #[zusi(id = 0x000B)]
    level_einstellen_anfordern: Option<u8>, // Client -> Zusi: ETCS-Level (STM, 0, 1, usw.)
    #[zusi(id = 0x000C)]
    stm_selected_index: Option<u16>, // Client -> Zusi
    #[zusi(id = 0x000D)]
    modus_einstellen_anfordern: Option<u16>, // Client -> Zusi
    #[zusi(id = 0x000E)]
    taf_modus: Option<u8>, // Client -> Zusi 1: TAF Quittiert 2: Grundstellung 3: TAF Abgelehnt
    #[zusi(id = 0x000F)]
    zugneustart: Option<u8>, // Zusi -> Client 1: Zug wurde neu gestartet bzw. neu uebernommen.
    #[zusi(id = 0x0010)]
    info_ton_abspielen: Option<u8>, // Client -> Zusi: 1: Zusi soll den Info-Ton 1x abspielen
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct EtcsStm {
    #[zusi(id = 0x0001)]
    stm_index: Option<u16>,
    #[zusi(id = 0x0002)]
    stm_name: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct EtcsZugdaten {
    #[zusi(id = 0x0001)]
    bremshundertstel: Option<u16>, // in %
    #[zusi(id = 0x0002)]
    zugkategorie: Option<u16>,
    #[zusi(id = 0x0003)]
    zuglaenge: Option<u16>, // in m
    #[zusi(id = 0x0004)]
    hoechstgeschwindigkeit: Option<u16>, // in km/h
    #[zusi(id = 0x0005)]
    achslast: Option<u16>, // in kg
    #[zusi(id = 0x0006)]
    zugnummer: Option<u16>,
    #[zusi(id = 0x0007)]
    tf_nummer: Option<u16>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct EtcsSpec {
    #[zusi(id = 0x0001)]
    reibwert: Option<u8>, // 1: vermindert 2: nicht vermindert
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct EtcsBetriebsdaten {
    #[zusi(id = 0x0001)]
    aktives_level: Option<u16>, // 0: Undefiniert 1: STM 2: 0 3: 1 4: 2 5: 3
    /*
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
    */
    #[zusi(id = 0x0002)]
    aktiver_modus: Option<u16>,
    #[zusi(id = 0x0003)]
    bremsungs_grund: Option<u16>,
    #[zusi(id = 0x0004)]
    bremsungs_grund_string: Option<String>,
    #[zusi(id = 0x0005)]
    stm_info: Option<EtcsStmInfo>,
    #[zusi(id = 0x0006)]
    level_ankuendigung: Option<EtcsLevelAnkuendigung>,
    #[zusi(id = 0x0007)]
    modus_ankuendigung: Option<EtcsModusAnkuendigung>,
    #[zusi(id = 0x0008)]
    funkstatus: Option<EtcsFunkstatus>,
    #[zusi(id = 0x0009)]
    zielgeschwindigkeit: Option<f32>, // in m/s
    #[zusi(id = 0x000A)]
    zielweg: Option<f32>, // in m, <0 = Dunkel
    #[zusi(id = 0x000B)]
    abstand_bremseinsatzpunkt: Option<f32>, // in m
    #[zusi(id = 0x000C)]
    entlassungsgeschwindigkeit: Option<f32>, // in m/s
    #[zusi(id = 0x000D)]
    sollgeschwindigkeit: Option<f32>, // in m/s
    #[zusi(id = 0x000E)]
    warngeschwindigkeit: Option<f32>, // in m/s
    #[zusi(id = 0x000F)]
    bremsgeschwindigkeit: Option<f32>, // in m/s
    #[zusi(id = 0x0010)]
    zwangsbremsgeschwindigkeit: Option<f32>, // in m/s
    #[zusi(id = 0x0011)]
    bremskurve_laeuft: Option<u8>, // 0: nein 1: ja
    #[zusi(id = 0x0012)]
    vorschaupunkte: Vec<EtcsVorschaupunkt>,
    #[zusi(id = 0x0013)]
    override_aktiv: Option<u8>, // Zusi -> Client
    #[zusi(id = 0x0014)]
    notruf_status: Option<u8>,
    #[zusi(id = 0x0015)]
    betriebszwangsbremsung: Option<u8>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct EtcsStmInfo {
    #[zusi(id = 0x0001)]
    stm_index: Option<u16>, // Index des aktiven STM-System, von 1 beginnend gemäß Reihenfolge in der ftd-Datei
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct EtcsLevelAnkuendigung {
    #[zusi(id = 0x0001)]
    neues_level: Option<u16>,
    #[zusi(id = 0x0002)]
    quittierung: Option<u8>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct EtcsModusAnkuendigung {
    #[zusi(id = 0x0001)]
    neuer_modus: Option<u16>,
    #[zusi(id = 0x0002)]
    quittierung: Option<u8>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct EtcsFunkstatus {
    #[zusi(id = 0x0001)]
    zustand: Option<u8>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct EtcsVorschaupunkt {
    #[zusi(id = 0x0001)]
    herkunft: Option<u16>, // 1: Strecke 3: Hauptsignal 9: Rangiersignal 14: ETCS
    #[zusi(id = 0x0002)]
    geschwindigkeit: Option<f32>, // in m/s, -1 = ETCS Ende
    #[zusi(id = 0x0003)]
    abstand: Option<f32>, // in m
    #[zusi(id = 0x0004)]
    hoehenwert: Option<f32>, // in m
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct ZubEinstellungen {
    #[zusi(id = 0x0001)]
    brh_wert: Option<u16>,
    #[zusi(id = 0x0002)]
    zuglaenge: Option<u16>,
    #[zusi(id = 0x0003)]
    vmax: Option<u16>, // in km/h
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct ZubBetriebsdaten {
    #[zusi(id = 0x0001)]
    lm_gnt: Option<u8>,
    #[zusi(id = 0x0002)]
    lm_gnt_ue: Option<u8>,
    #[zusi(id = 0x0003)]
    lm_gnt_g: Option<u8>,
    #[zusi(id = 0x0004)]
    lm_gnt_s: Option<u8>,
    #[zusi(id = 0x0005)]
    lm_gnt_gst: Option<u8>,
    #[zusi(id = 0x0006)]
    lm_gnt_stoer: Option<u8>,
    #[zusi(id = 0x0007)]
    status_lm_gnt_ue: Option<u8>,
    #[zusi(id = 0x0008)]
    status_lm_gnt_g: Option<u8>,
    #[zusi(id = 0x0009)]
    status_lm_gnt_s: Option<u8>,
    #[zusi(id = 0x000A)]
    zwangsbremsung: Option<u16>,
    #[zusi(id = 0x000B)]
    betriebsbremsung_aktiv: Option<u8>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
/// 5.3.3.3.5
pub struct StatusTuersystem {
    #[zusi(id = 0x0001)]
    pub bauart: Option<String>,
    #[zusi(id = 0x0002)]
    pub status_links: Option<u8>, // 0: zu 1: öffnend 2: offen 3: Fahrgastwechsel abgeschlossen 4: schließend 5: gestört 6: blockiert
    #[zusi(id = 0x0003)]
    pub status_rechts: Option<u8>, // 0: zu 1: öffnend 2: offen 3: Fahrgastwechsel abgeschlossen 4: schließend 5: gestört 6: blockiert
    #[zusi(id = 0x0004)]
    pub traktionssperre_aktiv: Option<u8>, // 1: an
    #[zusi(id = 0x0005)]
    pub seitenwahlschalter: Option<u8>, // 0: zu 1: links 2: rechts 3: beide
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
    pub status_zentrales_oeffnen_links: Option<u8>, // 0: Aus 1: Dauerlicht 2: Blinkend
    #[zusi(id = 0x0011)]
    pub status_zentrales_oeffnen_rechts: Option<u8>, // 0: Aus 1: Dauerlicht 2: Blinkend
    #[zusi(id = 0x0012)]
    pub lm_gruenschleife: Option<u8>, // 1: an
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
/// 5.3.3.3.6
pub struct StatusFahrzeug {
    #[zusi(id = 0x0001)]
    pub grund_nullstellungszwang: Option<u16>, // 0: nichts 1: niedriger HLL druck 2: dynamische Bremse 3: traktionssperre
    #[zusi(id = 0x0002)]
    pub grund_traktionssperre: Option<u16>, // 0: nichts 1: Federspeichenbremse 2: Türsystem 3: Bremsprobe läuft
    #[zusi(id = 0x0003)]
    pub status_fahrschalter: Option<u8>, // 1: deaktivert 2: normalzustand 3: gestört
    #[zusi(id = 0x0004)]
    pub status_dynamische_bremse: Option<u8>, // 1: deaktivert 2: normalzustand
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
    pub dateiname: Option<String>, // Dateiname des Zuges relativ zum Zusi-Verzeichnis. Wird ein Leerstring übermittelt, startet der zuletzt gefahrene Zug
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct EmptyNode {}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct Train {
    #[zusi(id = 0x0001)]
    pub zugnummer: Option<String>,
}
