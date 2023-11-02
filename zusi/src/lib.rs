use std::io::{Read, Write};

use either::Either;

use verbindungsaufbau::Verbindungsaufbau;
#[cfg(feature = "fahrpult")]
pub use zusi_fahrpult as fahrpult;
use zusi_protocol::{Deserialize, RootMessage, Serialize};

pub mod verbindungsaufbau;

struct TcpMessage<T>(Either<Verbindungsaufbau, T>)
where
    T: RootMessage;

impl<T> From<Verbindungsaufbau> for TcpMessage<T>
where
    T: RootMessage,
{
    fn from(msg: Verbindungsaufbau) -> Self {
        Self(Either::Left(msg))
    }
}

impl<T> From<T> for TcpMessage<T>
where
    T: RootMessage,
{
    fn from(value: T) -> Self {
        Self(Either::Right(value))
    }
}

impl<T> Serialize for TcpMessage<T>
where
    T: RootMessage,
{
    fn serialize<W>(&self, writer: &mut W, id: u16) -> zusi_protocol::Result<()>
    where
        W: Write,
    {
        match self.0 {
            Either::Left(ref msg) => Serialize::serialize(msg, writer, 0x0001)?,
            // TODO: extract id from T
            Either::Right(ref msg) => Serialize::serialize(msg, writer, 0x0002)?,
        }

        Ok(())
    }
}

impl<T> Deserialize for TcpMessage<T>
where
    T: RootMessage,
{
    fn deserialize<R>(reader: &mut R, _: u32) -> zusi_protocol::Result<Self>
    where
        R: Read,
    {
        let header = zusi_protocol::de::read_header(reader)?;
        let client_id = 0x0002;

        if let zusi_protocol::de::Header::Field { id, len: _ } = header {
            match id {
                0x0001 => {
                    let msg = Deserialize::deserialize(reader, 0)?;
                    Ok(Self(Either::Left(msg)))
                }
                client_id => {
                    let msg = Deserialize::deserialize(reader, 0)?;
                    Ok(Self(Either::Right(msg)))
                }
            }
        } else {
            Err(zusi_protocol::ProtocolError::Deserialization("expected field header".to_string()))
        }
    }
}
