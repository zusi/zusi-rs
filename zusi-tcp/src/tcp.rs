use byteorder;

use std::fmt;
use std::io;
use std::io::Read;
use std::io::Write;

use self::byteorder::{ReadBytesExt, WriteBytesExt};

use crate::node::Attribute;
use crate::node::Node;

type LE = self::byteorder::LittleEndian;

pub trait TcpSendable: fmt::Debug {
    fn send(&self, stream: &mut dyn Write) -> io::Result<()>;
}

impl TcpSendable for Node {
    fn send(&self, stream: &mut dyn Write) -> io::Result<()> {
        stream.write(&[0x00; 4])?; // PACKET_LENGTH == 0x00000000 denotes a node
        stream.write_u16::<LE>(self.id)?;
        for attr in &self.attributes {
            attr.send(stream)?;
        }
        for child in &self.children {
            child.send(stream)?;
        }
        stream.write(&[0xFF; 4])?;
        Ok(())
    }
}

impl TcpSendable for Attribute {
    fn send(&self, stream: &mut dyn Write) -> io::Result<()> {
        stream.write_u32::<LE>((self.value.len() as u32) + 2)?;
        stream.write_u16::<LE>(self.id)?;
        stream.write(&self.value)?;
        Ok(())
    }
}

pub fn receive(stream: &mut dyn Read) -> io::Result<Node> {
    let len = stream.read_u32::<LE>()?;
    assert_eq!(len, 0);
    receive_node(stream)
}

fn receive_node(stream: &mut dyn Read) -> io::Result<Node> {
    let id = stream.read_u16::<LE>()?;
    let mut result = Node {
        id: id,
        attributes: vec![],
        children: vec![],
    };
    loop {
        let len = stream.read_u32::<LE>()?;
        if len == 0x00000000 {
            // node
            match receive_node(stream) {
                Ok(node) => result.children.push(node),
                Err(e) => return Err(e),
            }
        } else if len == 0xFFFFFFFF {
            // end of node
            break;
        } else {
            // attribute
            let mut attr = Attribute {
                id: stream.read_u16::<LE>()?,
                value: Vec::new(),
            };
            stream
                .take((len - 2) as u64)
                .read_to_end(&mut attr.value)
                .ok()
                .expect("Error reading attribute value");
            result.attributes.push(attr);
        }
    }
    Ok(result)
}
