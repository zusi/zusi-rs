use nom::branch::alt;
use nom::number::streaming::{le_u16, le_u32};
use nom::{bytes::streaming::tag, IResult};
use zusi_protocol::de::Header;

pub fn header(input: &[u8]) -> IResult<&[u8], Header> {
    let (input, header) = alt((struct_end, field_header))(input)?;

    Ok((input, header))
}

fn struct_end(input: &[u8]) -> IResult<&[u8], Header> {
    const STRUCT_END: [u8; 4] = [0xFF, 0xFF, 0xFF, 0xFF];
    let (input, _) = tag(STRUCT_END)(input)?;

    Ok((input, Header::StructEnd))
}

fn field_header(input: &[u8]) -> IResult<&[u8], Header> {
    let (input, len) = le_u32(input)?;
    let (input, id) = le_u16(input)?;

    Ok((input, Header::Field { id, len }))
}

#[cfg(test)]
mod tests {
    use super::*;

    use nom::Err;

    #[test]
    fn test_header() {
        header(&vec![0xFF, 0xFF, 0xFF, 0xFF]).unwrap();
    }

    #[test]
    fn struct_end() {
        let (input, header) = header(&[0xFF, 0xFF, 0xFF, 0xFF]).unwrap();

        assert_eq!(input.len(), 0);
        assert_eq!(header, Header::StructEnd);
    }

    #[test]
    fn struct_end_incomplete() {
        match header(&[0xFF, 0xFF, 0xFF]) {
            Ok(_) => unreachable!(),
            Err(Err::Incomplete(_)) => {}
            Err(_) => unreachable!(),
        }
    }

    #[test]
    fn header_field() {
        let (input, header) = field_header(&[0x04, 0x00, 0x00, 0x00, 0x01, 0x00]).unwrap();

        assert_eq!(input.len(), 0);
        assert_eq!(header, Header::Field { id: 1, len: 4 });
    }

    #[test]
    fn header_node_begin() {
        let (input, header) = field_header(&[0x00, 0x00, 0x00, 0x00, 0x01, 0x00]).unwrap();

        assert_eq!(input.len(), 0);
        assert_eq!(header, Header::Field { id: 1, len: 0 });
    }
}
