use std::io::Write;

use nom::{
    branch::alt,
    bytes::streaming::{tag, take},
    number::streaming::{le_u16, le_u32},
    IResult,
};

const NODE_BEGIN: &[u8; 4] = &[0x00; 4];
const NODE_END: &[u8; 4] = &[0xFF; 4];

#[derive(Debug, Default)]
pub struct Node {
    pub id: u16,
    pub attributes: Vec<Attribute>,
    pub children: Vec<Node>,
}

impl Node {
    fn with_id(id: u16) -> Self {
        Self { id, ..Default::default() }
    }

    pub fn write<W>(&self, writer: &mut W) -> Result<(), std::io::Error>
    where W: Write,
    {
        writer.write_all(NODE_BEGIN)?;
        writer.write_all(&self.id.to_le_bytes())?;

        for c in &self.children {
            c.write(writer)?;
        }

        for a in &self.attributes {
            a.write(writer)?;
        }

        writer.write_all(NODE_END)?;
        todo!()
    }
}

#[derive(Debug)]
pub struct Attribute {
    pub id: u16,
    pub value: Vec<u8>,
}

impl Attribute {
    pub fn write<W>(&self, writer: &mut W) -> Result<(), std::io::Error>
    where W: Write,
    {
        writer.write_all(&(self.value.len() as u32).to_le_bytes())?;
        writer.write_all(&self.id.to_le_bytes())?;
        writer.write_all(&self.value)?;

        Ok(())
    }
}

pub fn read(input: &[u8]) -> IResult<&[u8], AttributeOrNode> {
    let (input, attr_or_node) = alt((node, end, attribute))(input)?;

    Ok((input, attr_or_node))
}

pub fn node(input: &[u8]) -> IResult<&[u8], AttributeOrNode> {
    let (input, _) = tag(NODE_BEGIN)(input)?;
    let (input, id) = le_u16(input)?;

    let mut node = Node::with_id(id);

    let mut input = input;
    loop {
        let (inp, elem) = read(input)?;
        input = inp;
        match elem {
            AttributeOrNode::End => return Ok((input, node.into())),
            AttributeOrNode::Attribute(a) => {
                node.attributes.push(a);
            }
            AttributeOrNode::Node(n) => {
                node.children.push(n);
            }
        }
    }
}

pub fn attribute(input: &[u8]) -> IResult<&[u8], AttributeOrNode> {
    let (input, len) = le_u32(input)?;
    let (input, id) = le_u16(input)?;
    let (input, value) = take(len - 2)(input)?;

    Ok((input, Attribute { id, value: value.to_vec() }.into()))
}

pub fn end(input: &[u8]) -> IResult<&[u8], AttributeOrNode> {
    let (input, _) = tag(NODE_END)(input)?;

    Ok((input, AttributeOrNode::End))
}

pub enum AttributeOrNode {
    Node(Node),
    Attribute(Attribute),
    End,
}

impl From<Node> for AttributeOrNode {
    fn from(n: Node) -> Self {
        Self::Node(n)
    }
}

impl From<Attribute> for AttributeOrNode {
    fn from(a: Attribute) -> Self {
        Self::Attribute(a)
    }
}
