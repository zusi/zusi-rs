use zusi_protocol::{Deserialize, Serialize};
use zusi_protocol_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
#[zusi(id = 0x0002)]
/// Anwendung 02 ("Fahrpult")
pub struct Fahrpult {
    #[zusi(id = 0x0001)]
    /// 5.3.3.1 Befehl 00 03 - NEEDED_DATA (Client → Zusi)
    pub needed_data: Option<NeededData>,
    #[zusi(id = 0x0002)]
    /// 5.3.3.2 Befehl 00 04 - ACK_NEEDED_DATA (Zusi → Client)
    pub ack_needed_data: Option<AckNeededData>,
    // NeededData    *NeededDataMessage `zusi:"0003" json:"needed_data,omitempty"`
    // AckNeededData *AckNeededData     `zusi:"0004" json:"ack_needed_data,omitempty"`
    // DataFtd       *DataFtd           `zusi:"000A" json:"data_ftd,omitempty"`
    // DataProg      *DataProg          `zusi:"000C" json:"data_prog,omitempty"`
    // Control       *Control           `zusi:"010B" json:"control,omitempty"`
}

#[derive(Serialize, Deserialize, Default, Debug)]
/// 5.3.3.1 Befehl 00 03 - NEEDED_DATA (Client → Zusi)
pub struct NeededData {
    #[zusi(id = 0x000A)]
    pub fuehrerstands_anzeigen: Option<FuehrerstandsAnzeigen>,
    #[zusi(id = 0x000B)]
    pub fuehrerstands_bedienung: Option<FuehrerstandsBedienung>,
    #[zusi(id = 0x000C)]
    pub programmdaten: Option<Programmdaten>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct FuehrerstandsAnzeigen {
    #[zusi(id = 0x0001)]
    pub anzeigen: Vec<u16>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct FuehrerstandsBedienung {}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Programmdaten {
    #[zusi(id = 0x0001)]
    pub anzeigen: Vec<u16>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
/// 5.3.3.2 Befehl 00 04 - ACK_NEEDED_DATA (Zusi → Client)
pub struct AckNeededData {
    #[zusi(id = 0x0001)]
    error_code: u8,
}

#[derive(Serialize, Deserialize, Default, Debug)]
/// 5.3.3.3 Befehl 00 0A - DATA_FTD (Zusi → Client)
pub struct DataFtd {
    geschwindigkeit: Option<f32>,          // Single: m/s
    druck_hauptluftleitung: Option<f32>,   // Single: bar
    druck_bremszylinder: Option<f32>,      // Single: bar
    druck_hauptluftbehaelter: Option<f32>, // Single: bar
    luftpresser_laeuft: Option<f32>,       // Single: aus/an
    luftstrom_fvb: Option<f32>,            // Single: -1..0..+1
    luftstrom_zbv: Option<f32>,            // Single: -1..0..+1
    luefter_an: Option<f32>,               // Single: aus/an
    zugkraft_gesamt: Option<f32>,          // Single: N
    zugkraft_pro_achse: Option<f32>,       // Single: N
    zugkraft_soll_gesamt: Option<f32>,     // Single: N
    zugkraft_soll_pro_achse: Option<f32>,  // Single: N
    oberstrom: Option<f32>,                // Single: A
    fahrleitungsspannung: Option<f32>,     // Single: V
    motordrehzahl: Option<f32>,            // Single: 1/min
    uhrzeit_stunde: Option<f32>,           // Single: Stunde (Zeigerposition Analoguhren)
    uhrzeit_minute: Option<f32>,           // Single: Minute (Zeigerposition Analoguhren)
    uhrzeit_sekunde: Option<f32>,          // Single: Sekunde (Zeigerposition Analoguhren)
    hauptschalter: Option<f32>,            // Single: aus/an
    trennschuetz: Option<f32>,             // Single: aus/an
    fahrstufe: Option<f32>,                // Single: 1
    //d3DFenster                                       : Option<f32>, // Single: Nicht sinnvoll zu verwenden
    afb_sollgeschwindigkeit: Option<f32>,   // Single: m/s
    druck_hilfsluftbehaelter: Option<f32>,  // Single: bar
    zurueckgelegter_gesamtweg: Option<f32>, // Single: m
    lm_getriebe: Option<f32>,               // Single: aus/an
    lm_schleudern: Option<f32>,             // Single: aus/an
    lm_gleiten: Option<f32>,                // Single: aus/an
    lm_mg_bremse: Option<f32>,              // Single: aus/an
    lm_hbremse: Option<f32>,                // Single: aus/an
    lm_rbremse: Option<f32>,                // Single: aus/an
    lm_hochabbremsung: Option<f32>,         // Single: aus/an
    lm_schnellbremsung: Option<f32>,        // Single: aus/an
    status_notbremssystem: Option<StatusNotbremssystem>,
    lm_uhrzeit_digital: Option<f32>, // Single: 0...1 (0:00 bis 24:00 Uhr)
    lm_drehzahlverstellung: Option<f32>, // Single: aus/an
    lm_fahrtrichtung_vor: Option<f32>, // Single: aus/an
    lm_fahrtrichtung_zurueck: Option<f32>, // Single: aus/an
    lm_fahrtrichtung_m: Option<f32>, // Single: aus/an
    //hintergrundbild                                 : Option<f32>,            // Single: Nicht sinnvoll zu verwenden
    motordrehmoment: Option<f32>,                    // Single: Nm
    motorlast_normiert: Option<f32>,                 // Single: 1 (0...1)
    tunnel: Option<f32>,                             // Single: aus/an
    schienenstoss_weiche: Option<f32>,               // Single: aus/an
    stahlbruecke: Option<f32>,                       // Single: aus/an
    steinbruecke: Option<f32>,                       // Single: aus/an
    xkoordinate: Option<f32>,                        // Single: m (Bez. Strecken-UTM-Punkt)
    ykoordinate: Option<f32>,                        // Single: m (Bez. Strecken-UTM-Punkt)
    zkoordinate: Option<f32>,                        // Single: m
    utm_referenzpunkt_xkm: Option<f32>,              // Single: km
    utmr_eferenzpunkt_ykm: Option<f32>,              // Single: km
    utm_zone: Option<f32>,                           // Single
    utm_zone2: Option<f32>,                          // Single
    afb_an: Option<f32>,                             // Single: aus/an
    individuell01: Option<f32>,                      // Single
    individuell02: Option<f32>,                      // Single
    individuell03: Option<f32>,                      // Single
    individuell04: Option<f32>,                      // Single
    individuell05: Option<f32>,                      // Single
    individuell06: Option<f32>,                      // Single
    individuell07: Option<f32>,                      // Single
    individuell08: Option<f32>,                      // Single
    individuell09: Option<f32>,                      // Single
    individuell10: Option<f32>,                      // Single
    individuell11: Option<f32>,                      // Single
    individuell12: Option<f32>,                      // Single
    individuell13: Option<f32>,                      // Single
    individuell14: Option<f32>,                      // Single
    individuell15: Option<f32>,                      // Single
    individuell16: Option<f32>,                      // Single
    individuell17: Option<f32>,                      // Single
    individuell18: Option<f32>,                      // Single
    individuell19: Option<f32>,                      // Single
    individuell20: Option<f32>,                      // Single
    datum: Option<f32>,                              // Single (Tage mit 0 = 30.12.1899)
    gleiskruemmung: Option<f32>,                     // Single 1000/m
    streckenhoechstgeschwindigkeit: Option<f32>,     // Single: m/s
    zugkraftvorschlag_autopilot: Option<f32>,        // Single: N
    beschleunigung_x: Option<f32>,                   // Single: m/s^2
    beschleunigung_y: Option<f32>,                   // Single: m/s^2
    beschleunigung_z: Option<f32>,                   // Single: m/s^2
    drehbeschleunigung_x_achse: Option<f32>,         // Single: rad/s^2
    drehbeschleunigung_y_achse: Option<f32>,         // Single: rad/s^2
    drehbeschleunigung_z_achse: Option<f32>,         // Single: rad/s^2
    stromabnehmer: Option<f32>, // Single: 2x4 bit (4bit: 1 fuer SA=oben; 4 bit: 1 fuer SA hebt sich gerade)
    lm_federspeicherbremse: Option<f32>, // Single: aus/an
    zustand_federspeicherbremse: Option<f32>, // Single -1, 0, 1, 2 (nicht vorhanden, aus, an, blinkend)
    steuerwagen_lm_getriebe: Option<f32>,     // Single: aus/an
    steuerwagen_lm_schleudern: Option<f32>,   // Single: aus/an
    steuerwagen_lm_gleiten: Option<f32>,      // Single: aus/an
    steuerwagen_lm_hbremse: Option<f32>,      // Single: aus/an
    steuerwagen_lm_rbremse: Option<f32>,      // Single: aus/an
    steuerwagen_lm_drehzahlverstellung: Option<f32>, // Single: aus/an
    druck_zweitbehaelter: Option<f32>,        // Single: bar
    geschwindigkeit_absolut: Option<f32>,     // Single: m/s
    zug_ist_entgleist: Option<f32>,           // Single: aus/an
    kilometrierung_zugspitze: Option<f32>,    // Single: km
    motorstrom: Option<f32>,                  // Single: A
    motorspannung: Option<f32>,               // Single: V
    status_sifa: Option<StatusSifa>,
    status_zugsicherung: Option<StatusZugbeeinflussung>,
    status_tuersystem: Option<StatusTuersystem>,
    individuell21: Option<f32>,                           // Single
    individuell22: Option<f32>,                           // Single
    individuell23: Option<f32>,                           // Single
    individuell24: Option<f32>,                           // Single
    individuell25: Option<f32>,                           // Single
    individuell26: Option<f32>,                           // Single
    individuell27: Option<f32>,                           // Single
    individuell28: Option<f32>,                           // Single
    individuell29: Option<f32>,                           // Single
    individuell30: Option<f32>,                           // Single
    individuell31: Option<f32>,                           // Single
    individuell32: Option<f32>,                           // Single
    individuell33: Option<f32>,                           // Single
    individuell34: Option<f32>,                           // Single
    individuell35: Option<f32>,                           // Single
    individuell36: Option<f32>,                           // Single
    individuell37: Option<f32>,                           // Single
    individuell38: Option<f32>,                           // Single
    individuell39: Option<f32>,                           // Single
    individuell40: Option<f32>,                           // Single
    steuerwagen_luefter_an: Option<f32>,                  // Single: aus/an
    steuerwagen_zugkraft_gesamt: Option<f32>,             // Single: N
    steuerwagen_zugraft_pro_achse: Option<f32>,           // Single: N
    steuerwagen_zugkraft_soll_gesamt: Option<f32>,        // Single: N
    steuerwagen_zugraft_soll_pro_achse: Option<f32>,      // Single: N
    steuerwagen_oberstrom: Option<f32>,                   // Single: N
    steuerwagen_fahrleitungsspannung: Option<f32>,        // Single V
    steuerwagen_motordrehzahl: Option<f32>,               // Single 1/min
    steuerwagen_hauptschalter: Option<f32>,               // Single aus/an
    steuerwagen_trennschuetz: Option<f32>,                // Single: aus/an
    steuerwagen_fahrstufe: Option<f32>,                   // Single: 1
    steuerwagen_motordrehmoment: Option<f32>,             // Single: Nm
    steuerwagen_motorlast_normiert: Option<f32>,          // Single: 1 (0...1)
    steuerwagen_stromabnehmer: Option<f32>,               // Single
    steuerwagen_motorstrom: Option<f32>,                  // Single: A
    steuerwagen_motorspannung: Option<f32>,               // Single: V
    geschwindigkeit_absolut_inkl_schleudern: Option<f32>, // Single: m/s
    batteriehauptschalter_aus: Option<f32>,               // Single: aus/an
    status_fahrzeug: Option<StatusFahrzeug>,
    status_zugverband: Option<StatusZugverband>,
    bremsprobenfunktion: Option<f32>, // Single: 0 (aus, >0 aktiv)
    zug_und_brems_gesamtkraft_soll_normiert: Option<f32>, // Single: 1 ((0...1) normiert auf aktuelle Fmax
    steuerwagen_zug_und_brems_gesamtkraft_soll_normiert: Option<f32>, // Single: 1 ((0...1) normiert auf aktuelle Fmax
    status_weichen: Option<StatusWeichen>,
    zug_und_brems_gesamtkraft_absolut_normiert: Option<f32>, // Single: 1 ((0...1) normiert auf aktuelle Fmax
    steuerwagen_zug_und_brems_gesamtkraft_absolut_normiert: Option<f32>, // Single: 1 ((0...1) normiert auf aktuelle Fmax
}

// 5.3.3.3.2
pub struct StatusNotbremssystem {
    bauart: Option<String>,
    status_notbremssystem: Option<u8>, // 0: NBÜ aus 1: NBÜ bereit 2: Notbremse gezogen 3: Notbremse wirkt (NBÜ bereit) 4: NBÜ durch Lokführer aktiviert 5 Notbremse wirkt (NBÜ aus)
    lm_system_bereit: Option<u8>,      // 1: an
    lm_notbremsung: Option<u8>,
    testmodus_aktiv: Option<u8>,
}

// 5.3.3.3.3
pub struct StatusSifa {
    bauart: Option<String>,
    lm_sifa: Option<u8>,
    sifa_hupe: Option<u8>,
    sifa_hauptschalter: Option<u8>,
    sifa_stoerschalter: Option<u8>,
    sifa_luftabsperrhahn: Option<u8>,
}

// 5.3.3.3.4 Status Zugbeeinflussung
pub struct StatusZugbeeinflussung {
    bauart: Option<String>,
    indusi_einstellungen: Option<IndusiEinstellungen>,
    indusi_zustand: Option<IndusiZustand>,
    etcs_einstellungen: Option<EtcsEinstellungen>,
    etcs_betriebsdaten: Option<EtcsBetriebsdaten>,
    zub_einstellungen: Option<ZubEinstellungen>,
    zub_betriebsdaten: Option<ZubBetriebsdaten>,
}

// 5.3.3.3.3.4.2 Indusi Analogsysteme und Basisdaten
pub struct IndusiEinstellungen {
    zugart: Option<u8>, // Zugart:1: Zugart muss noch bestimmt werden 2: U 3: M 4: O	5: S-Bahn-Modus
    tf_nummer: Option<String>,
    zugnummer: Option<String>,
    grunddaten: Option<Zugdaten>, // LZB
    ersatzzugdaten: Option<Zugdaten>,
    aktive_zugdaten: Option<Zugdaten>,
    hauptschalter: Option<u8>, // Hauptschalter   1: Indusi ausgeschaltet 2: Indusi eingeschaltet
    pzb_stoerschalter: Option<u8>, // Störschalter    1: Indusi abgeschaltet  2: Indusi eingeschaltet
    lzb_stoerschalter: Option<u8>, // LZB
    luftabsperrhahn: Option<u8>,   // Luftabsperrhahn 1: abgesperrt           2: offen
}

pub struct Zugdaten {
    bremshundertstel: Option<u16>,
    bremsart: Option<u16>,
    zuglaenge: Option<u16>, // LZB
    vmax: Option<u16>,      // LZB in km/h
    zugart: Option<u8>,
    modus: Option<u8>, // Nur relevant für AktiveZugdaten. 5: Ersatzzugdaten 6: Normalbetrieb
    klartextmeldungen: Option<u8>, // 0: Keine Klartextmeldungen möglich 1: Keine Klartextmeldungen möglich aber nicht aktiv 2: Klartextmeldungen aktiv 3: nur Klartextmeldungen möglich
}

pub struct IndusiZustand {
    zugsicherung: Option<u16>, // 1: Ausgeschaltet 2: abgeschaltet/gestört (1000Hz blinkt) 3: Hauptluftleitung unter 2,2 bar (1000Hz blinkt) 4: Aufforderung zur Zugdateneingabe 5: Normalbetrieb 6: Funktionsprüfung
    zwangsbremsungsgrund: Option<u16>, // 0: keine Zwangsbremsung, sonst Zwansbremsung aktiv wegen: 1. Wachsam 2. 1000Hz 3. 500Hz 4. 2000Hz 5. Kein Halt nach Befreiung aus Zwangsbremsung 6. Fahrzeug-v-Max überschritten 7. Funktionsprüfung 8. 500Hz nach Befreiung 9. LZB-Halt überfahren 10. LZB-Rechnerausfall 11. LZB-Nothalt überfahren 12. Übertragungsausfall in verdeckter Aufnahme
    zwangsbremsungsgrund_string: Option<String>,
    lm1000hz: Option<u8>,
    lm_u: Option<u8>,
    lm_m: Option<u8>,
    lm_o: Option<u8>,
    indusi_hupe: Option<u8>, // 1: Hupe 2: Zwangsbremsung
    lm500hz: Option<u8>,
    lm_befehl: Option<u8>,
    zusatzinfo_melderbild: Option<u8>, // PZB90 0: Normal 1: 1000Hz nach 700m 2: Restriktiv 3: Restriktiv+1000Hz 4: Restriktiv+500Hz 5: Prüfablauf nach LZB-Übertragungsausfall
    lzb: Option<u16>, // LZB 0: Keine LZB-Führung 1: Normale Fahrt 2: Nothalt 3: LZB-Halt überfahren 4: Rechnerausfall 5: Nachfahrauftrag 6: Funktionsprüfung
    lzb_ende_verfahren: Option<LzbAuftrag>,
    lzb_ersatzauftrag: Option<LzbAuftrag>,
    lzb_falschfahrauftrag: Option<LzbAuftrag>,
    lzb_vorsichtauftrag: Option<LzbAuftrag>,
    lzb_fahrt_ueber_halt_befehl: Option<LzbAuftrag>,
    lzb_uebertragungsausfall: Option<LzbUebertragungsausfall>,
    lzb_nothalt: Option<LzbNothalt>,
    lzb_rechnerausfall: Option<LzbRechnerausfall>,
    lzb_el_auftrag: Option<LzbElAuftrag>,
    lm_h: Option<u8>,
    lm_e40: Option<u8>,
    lm_ende: Option<u8>,
    lm_b: Option<u8>,
    lm_ue: Option<u8>,
    lm_g: Option<u8>,
    lm_el: Option<u8>,
    lm_v40: Option<u8>,
    lm_s: Option<u8>,
    lm_pruef_stoer: Option<u8>,
    sollgeschwindigkeit: Option<f32>, // m/s
    zielgeschwindigkeit: Option<f32>, // m/s Wert < 0 = Dunkel
    zielweg: Option<f32>,             // m Wert < 0 = Dunkel
    lm_gbl: Option<u8>,               // 0: aus 1: an 2: blinkt
    lm_pruef_stoer_bl: Option<u8>,    // 0: aus 1: an 2: blinkt
    cir_elke_modus: Option<u8>,       // 0: Normal 1: CIR-ELKE aktiv
    anzeigemodus: Option<u8>,         // 0: Normal 1: Zugdatenanzeige im MFA
    lzb_funktionspruefung: Option<LzbFunktionspruefung>,
    lm_zugart_links: Option<u8>,  // PZB90 S-Bahn
    lm_zugart65: Option<u8>,      // PZB90 S-Bahn
    lm_zugart_rechts: Option<u8>, // PZB90 S-Bahn
}

pub struct LzbAuftrag {
    status: Option<u8>, // 1: eingeleitet 2: quittiert bei Vorsichtauftrag: 3: Fahrt auf Sicht (V40 Melder Dauerlicht)
}

pub struct LzbUebertragungsausfall {
    zielgeschwindigkeit: Option<f32>, // m/s
    status: Option<u16>, // 1: eingeleitet 2: Ü Blinkt 3: erste Quittierung erfolt 4: Bedienung für 2. Quittierung gegeben 5: zweite Quittierung erfolgt 6: Ausfall nach verdeckter LZB Aufnahme (CE) 7: dito, Befehl blinkt
    zielweg: Option<f32>, // nur CIR-ELKE
}

pub struct LzbNothalt {
    status: Option<u8>,        // 1: empfangen 2: überfahren 3: aufgehoben
    wird_gesendet: Option<u8>, // 1: wird gesendet
}

pub struct LzbRechnerausfall {
    status: Option<u8>, // 1: alles dunkel 2: Befehlsmelder blinkt nach Neustart 3: Befehlsmelder Dauerlicht nach Quittierung
}

pub struct LzbElAuftrag {
    status: Option<u8>, // 1: Hauptschalter aus (EL Dauerlicht) 2: Stromabnehmer senken (EL Blinkt)
}

pub struct LzbFunktionspruefung {
    alle_melder_blinken: Option<EmptyNode>,
    anzeige_der_fuehrungsgroessen: Option<EmptyNode>,
    ban_ueaus: Option<EmptyNode>,
    zwangsbremsung_aktiv: Option<EmptyNode>,
}

pub struct EtcsEinstellungen {
    zustand: Option<u8>, // Zusi -> Client
    stm: Vec<EtcsStm>,   // Erstes STM = Aktives STM
    zugdaten: Option<EtcsZugdaten>,
    spec: Option<EtcsSpec>,
    etcs_stoerschalter: Option<u8>, // 1: ETCS Abgeschaltet 2: ETCS Eingeschaltet
    etcs_hauptschalter: Option<u8>, // 1: ETCS Abgeschaltet 2: ETCS Eingeschaltet
    luftabsperrhahn: Option<u8>,    // 1: Abgesperrt 2: Offen
    etcs_quittierschalter: Option<u8>, // 1: verlegt 2: grundstellung
    override_anforderung: Option<u8>, // 1: Override angefordert 2: Grundstellung
    start: Option<u8>,              // Nur Client -> Zusi : 1: Startkommando 2: Grundstellung
    level_einstellen_anfordern: Option<u8>, // Client -> Zusi: ETCS-Level (STM, 0, 1, usw.)
    stm_selected_index: Option<u16>, // Client -> Zusi
    modus_einstellen_anfordern: Option<u16>, // Client -> Zusi
    taf_modus: Option<u8>, // Client -> Zusi 1: TAF Quittiert 2: Grundstellung 3: TAF Abgelehnt
    zugneustart: Option<u8>, // Zusi -> Client 1: Zug wurde neu gestartet bzw. neu uebernommen.
    info_ton_abspielen: Option<u8>, // Client -> Zusi: 1: Zusi soll den Info-Ton 1x abspielen
}

pub struct EtcsStm {
    stm_index: Option<u16>,
    stm_name: Option<String>,
}

pub struct EtcsZugdaten {
    bremshundertstel: Option<u16>, // in %
    zugkategorie: Option<u16>,
    zuglaenge: Option<u16>,              // in m
    hoechstgeschwindigkeit: Option<u16>, // in km/h
    achslast: Option<u16>,               // in kg
    zugnummer: Option<u16>,
    tf_nummer: Option<u16>,
}

pub struct EtcsSpec {
    reibwert: Option<u8>, // 1: vermindert 2: nicht vermindert
}

pub struct EtcsBetriebsdaten {
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
    aktiver_modus: Option<u16>,
    bremsungs_grund: Option<u16>,
    bremsungs_grund_string: Option<String>,
    stm_info: Option<EtcsStmInfo>,
    level_ankuendigung: Option<EtcsLevelAnkuendigung>,
    modus_ankuendigung: Option<EtcsModusAnkuendigung>,
    funkstatus: Option<EtcsFunkstatus>,
    zielgeschwindigkeit: Option<f32>,        // in m/s
    zielweg: Option<f32>,                    // in m, <0 = Dunkel
    abstand_bremseinsatzpunkt: Option<f32>,  // in m
    entlassungsgeschwindigkeit: Option<f32>, // in m/s
    sollgeschwindigkeit: Option<f32>,        // in m/s
    warngeschwindigkeit: Option<f32>,        // in m/s
    bremsgeschwindigkeit: Option<f32>,       // in m/s
    zwangsbremsgeschwindigkeit: Option<f32>, // in m/s
    bremskurve_laeuft: Option<u8>,           // 0: nein 1: ja
    vorschaupunkte: Vec<EtcsVorschaupunkt>,
    override_aktiv: Option<u8>, // Zusi -> Client
    notruf_status: Option<u8>,
    betriebszwangsbremsung: Option<u8>,
}

pub struct EtcsStmInfo {
    stm_index: Option<u16>, // Index des aktiven STM-System, von 1 beginnend gemäß Reihenfolge in der ftd-Datei
}

pub struct EtcsLevelAnkuendigung {
    neues_level: Option<u16>,
    quittierung: Option<u8>,
}

pub struct EtcsModusAnkuendigung {
    neuer_modus: Option<u16>,
    quittierung: Option<u8>,
}

pub struct EtcsFunkstatus {
    zustand: Option<u8>,
}

pub struct EtcsVorschaupunkt {
    herkunft: Option<u16>, // 1: Strecke 3: Hauptsignal 9: Rangiersignal 14: ETCS
    geschwindigkeit: Option<f32>, // in m/s, -1 = ETCS Ende
    abstand: Option<f32>,  // in m
    hoehenwert: Option<f32>, // in m
}

pub struct ZubEinstellungen {
    brh_wert: Option<u16>,
    zuglaenge: Option<u16>,
    vmax: Option<u16>, // in km/h
}

pub struct ZubBetriebsdaten {
    lm_gnt: Option<u8>,
    lm_gnt_ue: Option<u8>,
    lm_gnt_g: Option<u8>,
    lm_gnt_s: Option<u8>,
    lm_gnt_gst: Option<u8>,
    lm_gnt_stoer: Option<u8>,
    status_lm_gnt_ue: Option<u8>,
    status_lm_gnt_g: Option<u8>,
    status_lm_gnt_s: Option<u8>,
    zwangsbremsung: Option<u16>,
    betriebsbremsung_aktiv: Option<u8>,
}

// 5.3.3.3.5
pub struct StatusTuersystem {
    bauart: Option<String>,
    status_links: Option<u8>, // 0: zu 1: öffnend 2: offen 3: Fahrgastwechsel abgeschlossen 4: schließend 5: gestört 6: blockiert
    status_rechts: Option<u8>, // 0: zu 1: öffnend 2: offen 3: Fahrgastwechsel abgeschlossen 4: schließend 5: gestört 6: blockiert
    traktionssperre_aktiv: Option<u8>, // 1: an
    seitenwahlschalter: Option<u8>, // 0: zu 1: links 2: rechts 3: beide
    lm_links: Option<u8>,
    lm_rechts: Option<u8>,
    status_lm_links: Option<u8>,
    status_lm_rechts: Option<u8>,
    lm_zwangsschliessen: Option<u8>,
    status_lm_zwangsschliessen: Option<u8>,
    lm_links_und_rechts: Option<u8>,
    status_lm_links_und_rechts: Option<u8>,
    zentrales_oeffnen_links: Option<u8>,
    zentrales_oeffnen_rechts: Option<u8>,
    status_zentrales_oeffnen_links: Option<u8>, // 0: Aus 1: Dauerlicht 2: Blinkend
    status_zentrales_oeffnen_rechts: Option<u8>, // 0: Aus 1: Dauerlicht 2: Blinkend
    lm_gruenschleife: Option<u8>,               // 1: an
}

// 5.3.3.3.6
pub struct StatusFahrzeug {
    grund_nullstellungszwang: Option<u16>, // 0: nichts 1: niedriger HLL druck 2: dynamische Bremse 3: traktionssperre
    grund_traktionssperre: Option<u16>, // 0: nichts 1: Federspeichenbremse 2: Türsystem 3: Bremsprobe läuft
    status_fahrschalter: Option<u8>,    // 1: deaktivert 2: normalzustand 3: gestört
    status_dynamische_bremse: Option<u8>, // 1: deaktivert 2: normalzustand
    status_sander: Option<u8>,
    status_bremsprobe: Option<u8>,
    stellung_richtungsschalter: Option<u8>,
}

// 5.3.3.3.7
pub struct StatusZugverband {
    status_fahrzeug: Vec<Einzelfahrzeug>,
}

pub struct Einzelfahrzeug {
    dateiname: Option<String>,
    beschreibung: Option<String>,
    bremsstellung: Option<u16>, // 0: keine/undefiniert 1: G 2: P 3 : P+Mg 4: R 5: R+Mg
    zugbeeinflussungssystem: Vec<EfzZugbeeinflussungssystem>,
    vmax: Option<f32>, // m/s
}

pub struct EfzZugbeeinflussungssystem {
    bezeichnung: Option<String>,
}

// 5.3.3.3.8 - Status Weichen
pub struct StatusWeichen {
    weichen: Vec<Weiche>,
}

pub struct Weiche {
    bezeichnung: Option<String>,
    bauart: Option<i32>,
    grundstellung: Option<i32>,
    lage: Option<u8>,
    fahrtrichtung: Option<u8>,
    umlaufmodus_stumpfbefahrung: Option<u8>,
}

// 5.3.3.4 Befehl 00 0B - DATA_OPERATION (Zusi → Client)

// 5.3.3.5 Befehl 00 0C - DATA_PROG (Zusi → Client)
pub struct DataProg {
    zugdatei: Option<String>,
    zugnummer: Option<String>,
    ladepause: Option<u8>,             // 0: Ende Ladepause
    buchfahrplandatei: Option<String>, // serialisertes xml
}

// 5.3.3.6 Befehl 01 0A - INPUT (Client → Zusi)

// 5.3.3.7 Befehl 01 0B - CONTROL (Client → Zusi)
pub struct Control {
    pause: Option<ShortNode>,
    restart: Option<ControlFilename>,
    start: Option<ControlFilename>,
    ende: Option<EmptyNode>,
    fpl_neustart: Option<EmptyNode>,
    zug_auswaehlen: Option<Train>, // nach Fahrplan neu starten
    zeitsprung: Option<ShortNode>,
    zeitraffer: Option<ShortNode>,
    nebel: Option<SingleNode>,
    helligkeit: Option<SingleNode>,
    reibwert: Option<SingleNode>,
    autopilot: Option<ShortNode>,
}

// 5.3.3.8 Befehl 01 0C - GRAPHIC (Client → Zusi)

pub struct ShortNode {
    control: Option<i16>, // -1: umschalten, 0: aus, 1: ein
}

pub struct SingleNode {
    control: Option<f32>,
}

pub struct ControlFilename {
    dateiname: Option<String>, // Dateiname des Zuges relativ zum Zusi-Verzeichnis. Wird ein Leerstring übermittelt, startet der zuletzt gefahrene Zug
}

pub struct EmptyNode {}

pub struct Train {
    zugnummer: Option<String>,
}
