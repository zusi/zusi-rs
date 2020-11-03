use std::io::Write;

use zusi_protocol::{Deserialize, Serialize};
use zusi_protocol_derive::{Deserialize, Serialize};

pub mod fahrpult;
/// Nachrichten welche zum Verbindungsaufbau zwischen Client und Zusi benutzt werden.
pub mod verbindungsaufbau;

#[derive(Serialize, Deserialize, Default, Debug)]
struct Message {
    #[zusi(id = 0x0001)]
    verbindungsaufbau: Option<verbindungsaufbau::Verbindungsaufbau>,
    #[zusi(id = 0x0002)]
    fahrpult: Option<fahrpult::Fahrpult>,
}

pub fn send_verbindungsaufbau<W>(msg: verbindungsaufbau::Verbindungsaufbau, mut writer: &mut W) -> Result<(), std::io::Error>
where W: Write {
    msg.serialize(&mut writer, 0)?;

    Ok(())
}
