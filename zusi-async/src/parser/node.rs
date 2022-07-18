use super::Attribute;

pub struct Node {
    pub id: u16,
    pub attributes: Vec<Attribute>,
    pub children: Vec<Node>,
}
