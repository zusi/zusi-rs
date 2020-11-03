use std::io::{Read, Write};
use std::net::TcpStream;
use std::net::ToSocketAddrs;

use thiserror::Error;

use zusi_protocol::{Deserialize, Serialize};
use zusi_protocol_derive::{Deserialize, Serialize};

use crate::fahrpult::Fahrpult;
use crate::verbindungsaufbau::{AckHello, Hello, Verbindungsaufbau};

pub mod fahrpult;
/// Nachrichten welche zum Verbindungsaufbau zwischen Client und Zusi benutzt werden.
pub mod verbindungsaufbau;

pub type Result<T> = std::result::Result<T, ZusiClientError>;

#[derive(Serialize, Deserialize, Default, Debug)]
struct Message {
    #[zusi(id = 0x0001)]
    verbindungsaufbau: Option<verbindungsaufbau::Verbindungsaufbau>,
    #[zusi(id = 0x0002)]
    fahrpult: Option<fahrpult::Fahrpult>,
}

pub fn send_verbindungsaufbau<W>(
    msg: verbindungsaufbau::Verbindungsaufbau,
    mut writer: &mut W,
) -> Result<()>
where
    W: Write,
{
    msg.serialize(&mut writer, 0)?;

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

pub fn receive_fahrpult<R>(mut reader: &mut R) -> Result<Fahrpult>
where
    R: Read,
{
    let msg: Message = Message::deserialize(&mut reader, 0)?;

    if let Some(m) = msg.fahrpult {
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
