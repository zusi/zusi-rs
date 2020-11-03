use std::io::BufReader;
use std::net::TcpStream;
use std::thread;

use zusi::{receive_fahrpult, ZusiClientError};

fn main() -> Result<(), ZusiClientError> {
    let (stream, ack) = zusi::connect("127.0.0.1:1436")?;

    println!("{}", ack.zusi_version);

    let reader = stream.try_clone()?;
    let handle = thread::spawn(move || {
        receiver_logger(reader).unwrap();
    });

    handle.join().unwrap();

    Ok(())
}

fn receiver_logger(stream: TcpStream) -> Result<(), ZusiClientError> {
    let mut stream = BufReader::new(stream);

    loop {
        let msg = receive_fahrpult(&mut stream)?;

        println!("{:?}", msg);
    }
}
