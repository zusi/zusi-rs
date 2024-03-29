//! The module defines the [`Message<T>`] type, as well as serialization and deserialization logic for it.

use std::io::{Read, Write};

use either::Either;

use crate::verbindungsaufbau::Verbindungsaufbau;
#[cfg(feature = "fahrpult")]
pub use zusi_fahrpult as fahrpult;
use zusi_protocol::{ClientType, Deserialize, ProtocolError, Serialize};

#[derive(Debug, PartialEq)]
/// A TCP message that can be sent or received.
/// The message can be either a [`Verbindungsaufbau`] message or a message of the specific client type (i.e. [`Fahrpult`]).
/// See [`
pub struct Message<T>(Either<Verbindungsaufbau, T>)
where
    T: ClientType;

impl<T> Message<T>
where
    T: ClientType,
{
    /// Reads and deserializes a TCP message from a reader.
    pub fn receive(mut reader: &mut impl Read) -> Result<Self, ProtocolError> {
        Self::deserialize(&mut reader, 0)
    }

    /// Serializes and writes the message to a writer.
    pub fn write(&self, writer: &mut impl Write) -> Result<(), ProtocolError> {
        self.serialize(writer, 0)
    }

    /// Returns `true` if the message is a `Verbindungsaufbau` message.
    pub fn is_verbindungsaufbau(&self) -> bool {
        matches!(self.0, Either::Left(_))
    }
}

impl<T> TryInto<Verbindungsaufbau> for Message<T>
where
    T: ClientType,
{
    type Error = ();

    fn try_into(self) -> Result<Verbindungsaufbau, Self::Error> {
        match self.0 {
            Either::Left(msg) => Ok(msg),
            _ => Err(()),
        }
    }
}

impl<T> From<Verbindungsaufbau> for Message<T>
where
    T: ClientType,
{
    fn from(msg: Verbindungsaufbau) -> Self {
        Self(Either::Left(msg))
    }
}

impl<T> From<T> for Message<T>
where
    T: ClientType,
{
    fn from(value: T) -> Self {
        Self(Either::Right(value))
    }
}

impl<T> Serialize for Message<T>
where
    T: ClientType,
{
    fn serialize<W>(&self, writer: &mut W, _: u16) -> zusi_protocol::Result<()>
    where
        W: Write,
    {
        match self.0 {
            Either::Left(ref msg) => Serialize::serialize(msg, writer, 0x0001)?,
            Either::Right(ref msg) => Serialize::serialize(msg, writer, T::ID)?,
        }

        Ok(())
    }
}

impl<T> Deserialize for Message<T>
where
    T: ClientType,
{
    fn deserialize<R>(reader: &mut R, _: u32) -> zusi_protocol::Result<Self>
    where
        R: Read,
    {
        let header = zusi_protocol::de::read_header(reader)?;
        let client_id = T::ID;

        if let zusi_protocol::de::Header::Field { id, len: _ } = header {
            if id == 0x0001 {
                let msg = Deserialize::deserialize(reader, 0)?;
                Ok(Self(Either::Left(msg)))
            } else if id == client_id {
                let msg = Deserialize::deserialize(reader, 0)?;
                Ok(Self(Either::Right(msg)))
            } else {
                Err(ProtocolError::Deserialization("unknown client id".to_string()))
            }
        } else {
            Err(ProtocolError::Deserialization("expected field header".to_string()))
        }
    }
}
