use std::io::{Read, Write};

pub(self) use crate::error::{Error, Result};

mod error;
pub mod ser;

fn write_u8<W: Write>(writer: W) -> Result<()> {
    unimplemented!()
}

fn read_u8<R: Read>(reader: R) -> Result<()> {
    unimplemented!()
}
