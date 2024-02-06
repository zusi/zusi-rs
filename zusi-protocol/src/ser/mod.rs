use std::io::Write;

use crate::Result;

mod num;

/// Serializes a Node or Attribute to an io::Writer.
pub trait Serialize {
    fn serialize<W>(&self, writer: &mut W, id: u16) -> Result<()>
    where
        W: Write;
}

impl<T> Serialize for Option<T>
where
    T: Serialize,
{
    fn serialize<W>(&self, writer: &mut W, id: u16) -> Result<()>
    where
        W: Write,
    {
        if let Some(s) = &self {
            s.serialize(writer, id)?;
        }

        Ok(())
    }
}

impl<T> Serialize for Vec<T>
where
    T: Serialize,
{
    fn serialize<W>(&self, writer: &mut W, id: u16) -> Result<()>
    where
        W: Write,
    {
        for elem in self {
            elem.serialize(writer, id)?;
        }

        Ok(())
    }
}

impl Serialize for String {
    fn serialize<W>(&self, writer: &mut W, id: u16) -> Result<()>
    where
        W: Write,
    {
        self.as_str().serialize(writer, id)
    }
}

impl Serialize for &str {
    fn serialize<W>(&self, writer: &mut W, id: u16) -> Result<()>
    where
        W: Write,
    {
        let len = self.len() as u32 + 2;

        writer.write_all(&len.to_le_bytes())?;
        writer.write_all(&id.to_le_bytes())?;
        writer.write_all(self.as_bytes())?;

        Ok(())
    }
}

pub fn write_node_header<W>(writer: &mut W, id: u16) -> Result<()>
where
    W: Write,
{
    writer.write_all(&crate::NODE_START)?;
    writer.write_all(&id.to_le_bytes())?;

    Ok(())
}

pub fn write_node_end<W>(writer: &mut W) -> Result<()>
where
    W: Write,
{
    writer.write_all(&crate::NODE_END)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::to_bytes;

    #[test]
    fn test_string() {
        let input = "Bla".to_string();

        let expected: Vec<u8> = vec![5, 0, 0, 0, 1, 0, 66, 108, 97];

        assert_eq!(to_bytes(&input).unwrap(), expected);
    }

    #[test]
    fn test_str() {
        let input = "Bla";

        let expected: Vec<u8> = vec![5, 0, 0, 0, 1, 0, 66, 108, 97];

        assert_eq!(to_bytes(&input).unwrap(), expected);
    }
}
