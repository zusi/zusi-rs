pub struct Node {
    pub id: u16,
    pub attributes: Vec<Attribute>,
    pub children: Vec<Node>,
}

pub struct Attribute {
    pub id: u16,
    pub value: Vec<u8>,
}
