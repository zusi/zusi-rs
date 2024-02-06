use either::Either;
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
    pub fn with_id(id: u16) -> Self {
        Self { id, ..Default::default() }
    }
}

#[derive(Debug)]
pub struct Attribute {
    pub id: u16,
    pub value: Vec<u8>,
}

pub fn read(input: &[u8]) -> IResult<&[u8], Option<AttrOrNode>> {
    let (input, attr_or_node) = alt((node, end, attribute))(input)?;

    Ok((input, attr_or_node))
}

pub fn node(input: &[u8]) -> IResult<&[u8], Option<AttrOrNode>> {
    let (input, _) = tag(&[0x00, 0x00, 0x00, 0x00])(input)?;
    let (input, id) = le_u16(input)?;

    let mut node = Node::with_id(id);

    let mut input = input;
    loop {
        let (inp, elem) = read(input)?;
        input = inp;
        if let Some(elem) = elem {
            match elem {
                Either::Left(a) => node.attributes.push(a),
                Either::Right(n) => node.children.push(n),
            }
        } else {
            return Ok((input, Some(node.into())));
        }
    }
}

pub fn attribute(input: &[u8]) -> IResult<&[u8], Option<AttrOrNode>> {
    let (input, len) = le_u32(input)?;
    let (input, id) = le_u16(input)?;
    let (input, value) = take(len - 2)(input)?;

    Ok((input, Some(Attribute { id, value: value.to_vec() }.into())))
}

pub fn end(input: &[u8]) -> IResult<&[u8], Option<AttrOrNode>> {
    let (input, _) = tag(&[0xFF, 0xFF, 0xFF, 0xFF])(input)?;

    Ok((input, None))
}

pub type AttrOrNode = Either<Attribute, Node>;

impl From<Node> for AttrOrNode {
    fn from(n: Node) -> Self {
        Self::Right(n)
    }
}

impl From<Attribute> for AttrOrNode {
    fn from(a: Attribute) -> Self {
        Self::Left(a)
    }
}
