use std::io::{BufReader, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::thread;

use thiserror::Error;
use zusi::verbindungsaufbau::*;
use zusi::{fahrpult::*, Message};
use zusi_protocol::{Deserialize, Serialize};

fn main() -> Result<()> {
    let (mut stream, ack) = connect("127.0.0.1:1436")?;

    println!("{}", ack.zusi_version);

    let reader = stream.try_clone()?;
    let handle = thread::spawn(move || {
        receiver_logger(reader).unwrap();
    });

    {
        let needed_data = NeededData {
            fuehrerstands_anzeigen: Some(FuehrerstandsAnzeigen { anzeigen: vec![0x0001] }),
            ..Default::default()
        };
        let needed_data = Fahrpult { needed_data: Some(needed_data), ..Default::default() };

        send_fahrpult(needed_data, &mut stream)?;
    }

    handle.join().unwrap();

    Ok(())
}

fn receiver_logger(stream: TcpStream) -> Result<()> {
    let mut stream = BufReader::new(stream);

    loop {
        let msg = receive_fahrpult(&mut stream)?;

        println!("{:?}", msg);
    }
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
    let handshake = Verbindungsaufbau { hello: Some(handshake), ..Default::default() };
    handshake.serialize(&mut stream, 0)?;

    // Receive ACK
    let msg = receive_verbindungsaufbau(&mut stream)?;
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

pub type Result<T> = std::result::Result<T, ZusiClientError>;
pub type FahrpultMessage = Message<Fahrpult, 2>;

fn receive_message<R: Read>(mut reader: &mut R) -> Result<FahrpultMessage> {
    Ok(Message::deserialize(&mut reader, 0)?)
}

pub fn receive_fahrpult<R: Read>(mut reader: &mut R) -> Result<Fahrpult> {
    let msg: Message<Fahrpult, 2> = receive_message(&mut reader)?;

    if let Some(m) = msg.message {
        return Ok(m);
    }

    Err(ZusiClientError::WrongMessageType)
}

pub fn receive_verbindungsaufbau<R: Read>(mut reader: &mut R) -> Result<Verbindungsaufbau> {
    let msg: Message<Fahrpult, 2> = receive_message(&mut reader)?;

    if let Some(m) = msg.verbindungsaufbau {
        return Ok(m);
    }

    Err(ZusiClientError::WrongMessageType)
}

pub fn send_verbindungsaufbau<W>(msg: Verbindungsaufbau, mut writer: &mut W) -> Result<()>
where
    W: Write,
{
    msg.serialize(&mut writer, 1)?;

    Ok(())
}

pub fn send_fahrpult<W>(msg: Fahrpult, mut writer: &mut W) -> Result<()>
where
    W: Write,
{
    msg.serialize(&mut writer, 2)?;

    Ok(())
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
