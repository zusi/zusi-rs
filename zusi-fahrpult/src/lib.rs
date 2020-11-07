use std::io::{Read, Write};
use std::net::TcpStream;
use std::net::ToSocketAddrs;

use thiserror::Error;

use zusi_protocol::de::Header;
use zusi_protocol::{Deserialize, Serialize};

use crate::fahrpult::Fahrpult;
use crate::verbindungsaufbau::{AckHello, Hello, Verbindungsaufbau};

pub mod fahrpult;
/// Nachrichten welche zum Verbindungsaufbau zwischen Client und Zusi benutzt werden.
pub mod verbindungsaufbau;

#[cfg(test)]
mod integration_test;

pub type Result<T> = std::result::Result<T, ZusiClientError>;

#[derive(Default, Debug, PartialEq)]
struct Message {
    // #[zusi(id = 0x0001)]
    verbindungsaufbau: Option<verbindungsaufbau::Verbindungsaufbau>,
    // #[zusi(id = 0x0002)]
    fahrpult: Option<fahrpult::Fahrpult>,
}

impl Serialize for Message {
    fn serialize<W>(&self, writer: &mut W, _: u16) -> std::result::Result<(), std::io::Error>
    where
        W: Write,
    {
        Serialize::serialize(&self.verbindungsaufbau, writer, 0x0001)?;
        Serialize::serialize(&self.fahrpult, writer, 0x0002)?;

        Ok(())
    }
}

impl Deserialize for Message {
    fn deserialize<R>(reader: &mut R, _: u32) -> std::result::Result<Self, std::io::Error>
    where
        R: Read,
    {
        let mut node: Self = Default::default();

        let header = zusi_protocol::de::read_header(reader)?;

        if let Header::Field { id, len: _ } = header {
            match id {
                0x0001 => {
                    Deserialize::deserialize_in_place(reader, 0, &mut node.verbindungsaufbau)?
                }
                0x0002 => Deserialize::deserialize_in_place(reader, 0, &mut node.fahrpult)?,
                _ => {}
            }
        }

        Ok(node)
    }
}

pub fn send_verbindungsaufbau<W>(
    msg: verbindungsaufbau::Verbindungsaufbau,
    mut writer: &mut W,
) -> Result<()>
where
    W: Write,
{
    msg.serialize(&mut writer, 1)?;

    Ok(())
}

pub fn send_fahrpult<W>(msg: fahrpult::Fahrpult, mut writer: &mut W) -> Result<()>
where
    W: Write,
{
    msg.serialize(&mut writer, 2)?;

    Ok(())
}

pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<(TcpStream, AckHello)> {
    let mut stream = TcpStream::connect(addr)?;

    // Send handshake message
    let handshake = Hello {
        protokoll_version: 2,
        client_typ: 2,
        name: "Fahrpult".to_string(),
        version: "2.0".to_string(),
    };
    let handshake = Verbindungsaufbau {
        hello: Some(handshake),
        ack_hello: None,
    };
    handshake.serialize(&mut stream, 0)?;

    // Receive ACK
    let ack: AckHello = AckHello::deserialize_struct(&mut stream)?;

    if ack.error_code != 0 {
        return Err(ZusiClientError::Connect {
            error_code: ack.error_code,
        });
    }

    Ok((stream, ack))
}

fn receive_message<R: Read>(mut reader: &mut R) -> Result<Message> {
    Ok(Message::deserialize(&mut reader, 0)?)
}

pub fn receive_fahrpult<R: Read>(mut reader: &mut R) -> Result<Fahrpult> {
    let msg: Message = receive_message(&mut reader)?;

    if let Some(m) = msg.fahrpult {
        return Ok(m);
    }

    Err(ZusiClientError::WrongMessageType)
}

pub fn receive_verbindungsaufbau<R: Read>(mut reader: &mut R) -> Result<Verbindungsaufbau> {
    let msg: Message = receive_message(&mut reader)?;

    if let Some(m) = msg.verbindungsaufbau {
        return Ok(m);
    }

    Err(ZusiClientError::WrongMessageType)
}

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum ZusiClientError {
    #[error("IO")]
    Io {
        #[from]
        source: std::io::Error,
    },
    #[error("error code returned from server {error_code}")]
    Connect { error_code: u8 },
    #[error("other message type as tried to receive was received")]
    WrongMessageType,
}
