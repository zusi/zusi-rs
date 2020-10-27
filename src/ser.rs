use std::io::Write;

use crate::Result;

pub fn to_bytes<T>(value: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    // let mut serializer = Serializer {
    //     output: Default::default(),
    // };
    // value.serialize(&mut serializer)?;
    let mut c = Vec::new();

    value.serialize(&mut c);

    Ok(c)
}

////////////////////////////////////////////////////////////////////////////////

pub trait Serialize {
    fn serialize<W>(&self, writer: &mut W)
    where
        W: Write;
}

#[cfg(test)]
mod tests {
    use zusi_protocol::Serialize;

    use super::to_bytes;
    use super::Serialize;

    #[allow(dead_code)]
    #[test]
    #[rustfmt::skip]
    fn test_struct() {
        #[derive(Serialize)]
        struct Test {
            #[zusi(id=0x0001)]
            protokoll_version: u16,
            #[zusi(id=0x0021)]
            client_typ: u16,
            name: String,
            version: String,
        }

        let test = Test {
            protokoll_version: 2,
            client_typ: 2,
            name: "Fahrpult".to_string(),
            version: "2.0".to_string(),
        };

        let expected: Vec<u8> = vec![
            0x04, 0x00, 0x00, 0x00, // Länge 4 Bytes → es folgt ein Attribut, Länge 4 bytes
            0x01, 0x00, // ID x0001: Protokoll-Version
            0x02, 0x00, // Protokoll-Version "2" (Word)
            0x04, 0x00, 0x00, 0x00, // Länge 4 Bytes → es folgt ein Attribut, Länge 4 bytes
            0x02, 0x00, // ID x0002: Client-Typ
            0x02, 0x00, // Client-Typ "Fahrpult" (Word)
            0x0A, 0x00, 0x00, 0x00, // Länge 10 Bytes → es folgt ein Attribut
            0x03, 0x00, // ID x0003: Klartextstring
            0x46, 0x61, 0x68, 0x72, 0x70, 0x75, 0x6C, 0x74, // String "Fahrpult" (8 Zeichen, da 2 bytes für die ID)
            0x05, 0x00, 0x00, 0x00, // Länge 5 Bytes → es folgt ein Attribut
            0x04, 0x00, // ID x0004: Version
            0x32, 0x2E, 0x30, // String "2.0"
        ];
        assert_eq!(to_bytes(&test).unwrap(), expected);
    }
}
