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
