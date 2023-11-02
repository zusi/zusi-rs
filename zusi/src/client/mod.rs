#[cfg(feature = "async")]
pub mod codec;

use crate::verbindungsaufbau::{AckHello, Hello, Verbindungsaufbau};
use crate::Message;
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use thiserror::Error;
use zusi_protocol::{ClientType, Serialize};

pub fn send_verbindungsaufbau<W>(
    msg: Verbindungsaufbau,
    mut writer: &mut W,
) -> Result<(), ZusiClientError>
where
    W: Write,
{
    msg.serialize(&mut writer, 1)?;

    Ok(())
}

pub fn connect<A: ToSocketAddrs, T: ClientType>(
    addr: A,
) -> Result<(TcpStream, AckHello), ZusiClientError> {
    let mut stream = TcpStream::connect(addr)?;

    // Send handshake message
    let handshake = Hello {
        protokoll_version: 2,
        client_typ: 2,
        name: "Fahrpult".to_string(),
        version: "2.0".to_string(),
    };
    let handshake = Verbindungsaufbau { hello: Some(handshake), ..Default::default() };
    handshake.serialize(&mut stream, 0)?;

    // Receive ACK
    let msg = receive_verbindungsaufbau::<_, T>(&mut stream)?;
    let ack = if let Some(msg) = msg.ack_hello {
        msg
    } else {
        return Err(ZusiClientError::WrongMessageType);
    };

    if ack.error_code != 0 {
        return Err(ZusiClientError::Connect { error_code: ack.error_code });
    }

    Ok((stream, ack))
}

pub fn receive_verbindungsaufbau<R: Read, T: ClientType>(
    mut reader: &mut R,
) -> Result<Verbindungsaufbau, ZusiClientError> {
    let msg = Message::<T>::receive(&mut reader)?;

    msg.try_into().or(Err(ZusiClientError::WrongMessageType))
}

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum ZusiClientError {
    #[error("IO")]
    Io {
        #[from]
        source: std::io::Error,
    },
    #[error("TCP Client")]
    Protocol {
        #[from]
        soruce: zusi_protocol::ProtocolError,
    },
    #[error("error code returned from server {error_code}")]
    Connect { error_code: u8 },
    #[error("other message type as tried to receive was received")]
    WrongMessageType,
}
