use std::net::TcpStream;

use zusi_protocol::{Deserialize, Serialize};
use zusi_protocol_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
struct HelloMsg {
    #[zusi(id = 0x0001)]
    protokoll_version: u16,
    #[zusi(id = 0x0002)]
    client_typ: u16,
    #[zusi(id = 0x0003)]
    name: String,
    #[zusi(id = 0x0004)]
    version: String,
}

fn main() -> Result<(), std::io::Error> {
    let mut stream = TcpStream::connect("127.0.0.1:1435")?;

    let msg: HelloMsg = Deserialize::deserialize_struct(&mut stream).unwrap();

    println!("{:?}", msg);

    Ok(())
}
