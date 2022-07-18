mod attribute;
mod header;
mod node;

pub use attribute::Attribute;
pub use node::Node;
use nom::IResult;

pub fn read(input: &[u8]) -> IResult<&[u8], usize> {
    todo!();
}
