use std::net::TcpStream;

use zusi::verbindungsaufbau::{AckHello, Hello, Verbindungsaufbau};
use zusi_protocol::{Deserialize};

fn main() -> Result<(), std::io::Error> {
    let mut stream = TcpStream::connect("127.0.0.1:1436")?;

    let hello = Hello {
        protokoll_version: 2,
        client_typ: 2,
        name: "Fahrpult".to_string(),
        version: "2.0".to_string(),
    };

    let hello = Verbindungsaufbau {
        hello: Some(hello),
        ack_hello: None,
    };

    zusi::send_verbindungsaufbau(hello, &mut stream)?;
    let msg: AckHello = Deserialize::deserialize_struct(&mut stream)?;

    println!("{:?}", msg);

    Ok(())
}
