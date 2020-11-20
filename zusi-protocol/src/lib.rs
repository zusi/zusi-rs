use std::io;
use std::io::Write;

pub use crate::de::Deserialize;
pub use crate::ser::Serialize;

pub mod de;
pub mod ser;

pub type Result<T> = core::result::Result<T, io::Error>;

pub const NODE_START: [u8; 4] = [0; 4];
pub const NODE_END: [u8; 4] = [0xFF; 4];

pub fn to_bytes<T>(value: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    let mut c = Vec::new();

    value.serialize(&mut c, 0x01)?;
    c.flush()?;

    Ok(c)
}