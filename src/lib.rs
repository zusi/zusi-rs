use std::io::{Read, Write};

use num_traits::Num;

pub(self) use crate::error::{Error, Result};

mod error;
pub mod ser;

fn write_u8<W: Write>(writer: &mut W, value: u8) -> Result<()> {
    let bts = value.to_le_bytes();

    writer.write_all(&bts)?;

    Ok(())
}

fn read_u8<R: Read>(reader: &mut R) -> Result<()> {
    unimplemented!()
}


fn write<W, N>(writer: &mut W, value: N) -> Result<()>
    where W: Write,
          N: Num {
    Ok(())
}