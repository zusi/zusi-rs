use std::net::TcpStream;

use zusi::verbindungsaufbau::{AckHello, Hello};
use zusi_protocol::{Deserialize, Serialize};

fn main() -> Result<(), std::io::Error> {
    let mut stream = TcpStream::connect("127.0.0.1:1436")?;

    let hello = Hello {
        protokoll_version: 2,
        client_typ: 2,
        name: "Fahrpult".to_string(),
        version: "2.0".to_string(),
    };

    Serialize::serialize(&hello, &mut stream, 0x0001)?;
    let msg: AckHello = Deserialize::deserialize_struct(&mut stream)?;

    println!("{:?}", msg);

    Ok(())
}
