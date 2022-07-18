use nom::{
    branch::alt,
    bytes::streaming::{tag, take},
    number::streaming::{le_u16, le_u32},
    IResult,
};

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
}

#[derive(Debug)]
pub struct Attribute {
    pub id: u16,
    pub value: Vec<u8>,
}

pub fn read(input: &[u8]) -> IResult<&[u8], AttributeOrNode> {
    let (input, attr_or_node) = alt((node, end, attribute))(input)?;

    return Ok((input, attr_or_node));
}

pub fn node(input: &[u8]) -> IResult<&[u8], AttributeOrNode> {
    let (input, _) = tag(&[0x00, 0x00, 0x00, 0x00])(input)?;
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
    let (input, _) = tag(&[0xFF, 0xFF, 0xFF, 0xFF])(input)?;

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
