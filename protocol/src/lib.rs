use std::io;
use std::io::Write;

use crate::ser::write_node_header;
pub use crate::ser::Serialize;

pub mod ser;

pub type Result<T> = core::result::Result<T, io::Error>;

pub const NODE_START: [u8; 4] = [0; 4];
pub const NODE_END: [u8; 4] = [0xFF; 4];

pub fn bla(input: u16) {
    let bts = to_bytes(&input).unwrap();

    let expected: Vec<u8> = vec![0x04, 0x00, 0x00, 0x00, 0x02, 0x00, 0x05, 0x00];

    assert_eq!(bts, expected);
}

struct TestMessage {
    field: u16, // 0x0001
}

impl Serialize for TestMessage {
    fn serialize<W>(&self, writer: &mut W, id: u16) -> Result<()>
    where
        W: Write,
    {
        write_node_header(writer, id)?;
        // node header
        writer.write_all(&NODE_START)?;
        writer.write_all(&id.to_le_bytes())?;

        // for each field:
        Serialize::serialize(&self.field, writer, 0x0001)?;

        // node end
        writer.write_all(&NODE_END)?;

        Ok(())
    }
}

pub fn to_bytes<T>(value: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    let mut c = Vec::new();

    value.serialize(&mut c, 0x01)?;
    c.flush()?;

    Ok(c)
}
