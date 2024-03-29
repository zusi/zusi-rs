//! Client implementation for Zusi 3 TCP-Client.
//! An async version for the tokio runtime is available when enabling the `async` feature.

#[cfg(feature = "async")]
mod codec;

#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
#[cfg(feature = "async")]
pub use codec::ZusiProtocolCodec;

use crate::verbindungsaufbau::{AckHello, Hello, Verbindungsaufbau};
use crate::Message;
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use thiserror::Error;
use zusi_protocol::{ClientType, Serialize};

/// Send a Verbindungsaufbau message to the server.
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

/// Connect to a Zusi 3 server.
///
/// # Example
/// ```no_run
/// # use zusi::client::connect;
/// # use zusi::Message;
/// # use zusi_fahrpult::{Fahrpult, FuehrerstandsAnzeigen, NeededData};
/// let (mut stream, ack) = connect::<Fahrpult>("127.0.0.1:1435")?;
/// println!("{}", ack.zusi_version);
///
/// // Send a message
/// let needed_data = NeededData {
///     fuehrerstands_anzeigen: Some(FuehrerstandsAnzeigen { anzeigen: vec![0x0001] }),
///     ..Default::default()
/// };
/// let needed_data = Fahrpult { needed_data: Some(needed_data), ..Default::default() };
/// Message::write(&needed_data.into(), &mut stream)?;
///
/// // Receive a message
/// let msg: Message<Fahrpult> = Message::receive(&mut stream)?;
/// ```
pub fn connect<T: ClientType>(
    addr: impl ToSocketAddrs,
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

/// Receive a Verbindungsaufbau message from the server.
/// It is safe to assert that the first message received from the server is a Verbindungsaufbau message.
pub fn receive_verbindungsaufbau<R: Read, T: ClientType>(
    mut reader: &mut R,
) -> Result<Verbindungsaufbau, ZusiClientError> {
    let msg = Message::<T>::receive(&mut reader)?;

    msg.try_into().or(Err(ZusiClientError::WrongMessageType))
}

#[non_exhaustive]
#[derive(Error, Debug)]
#[allow(missing_docs)]
/// Error type for the Zusi 3 TCP-Client.
pub enum ZusiClientError {
    #[error("IO")]
    Io(#[from] std::io::Error),
    #[error("TCP Client")]
    Protocol(#[from] zusi_protocol::ProtocolError),
    #[error("error code returned from server {error_code}")]
    Connect { error_code: u8 },
    #[error("other message type as tried to receive was received")]
    WrongMessageType,
}
