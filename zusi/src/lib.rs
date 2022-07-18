use std::marker::PhantomData;

use zusi_protocol::{de::Header, Deserialize, RootMessage, Serialize};

pub mod verbindungsaufbau;

pub mod fahrpult {
    pub use zusi_fahrpult::fahrpult::*;
}

pub mod protocol {
    pub use zusi_protocol::*;
}

#[derive(Default)]
pub struct Client<M>
where
    M: RootMessage,
{
    phantom: PhantomData<*const M>,
}

impl<M> Client<M>
where
    M: RootMessage,
{
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct Message<T: Serialize + Deserialize + PartialEq, const ID: u16> {
    pub verbindungsaufbau: Option<verbindungsaufbau::Verbindungsaufbau>,
    pub message: Option<T>,
}

impl<T: Serialize + Deserialize + PartialEq, const ID: u16> Serialize for Message<T, ID> {
    fn serialize<W>(&self, writer: &mut W, _: u16) -> zusi_protocol::Result<()>
    where
        W: std::io::Write,
    {
        Serialize::serialize(&self.verbindungsaufbau, writer, 0x0001)?;
        Serialize::serialize(&self.message, writer, ID)?;

        Ok(())
    }
}

impl<T: Serialize + Deserialize + PartialEq, const ID: u16> Deserialize for Message<T, ID> {
    fn deserialize<R>(reader: &mut R, _: u32) -> zusi_protocol::Result<Self>
    where
        R: std::io::Read,
    {
        let mut node: Self = Default::default();

        let header = zusi_protocol::de::read_header(reader)?;

        if let Header::Field { id, len: _ } = header {
            if id == 0x0001 {
                Deserialize::deserialize_in_place(reader, 0, &mut node.verbindungsaufbau)?
            } else if id == ID {
                Deserialize::deserialize_in_place(reader, 0, &mut node.message)?
            }
        }

        Ok(node)
    }
}

impl<T: Serialize + Deserialize + PartialEq, const ID: u16> RootMessage for Message<T, ID> {
    fn id(&self) -> u16 {
        ID
    }
}
