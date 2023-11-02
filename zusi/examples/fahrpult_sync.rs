use std::io::BufReader;
use std::net::TcpStream;
use std::thread;
use zusi::client::{connect, ZusiClientError};
use zusi::fahrpult::{Fahrpult, FuehrerstandsAnzeigen, NeededData};
use zusi::Message;

fn main() -> Result<(), ZusiClientError> {
    let (mut stream, ack) = connect::<_, Fahrpult>("10.211.55.3:1436")?;

    println!("{}", ack.zusi_version);

    let reader = stream.try_clone()?;
    let handle = thread::spawn(move || {
        receiver_logger(reader).unwrap();
    });

    {
        let needed_data = NeededData {
            fuehrerstands_anzeigen: Some(FuehrerstandsAnzeigen { anzeigen: vec![0x0001] }),
            ..Default::default()
        };
        let needed_data = Fahrpult { needed_data: Some(needed_data), ..Default::default() };

        Message::write(&needed_data.into(), &mut stream)?;
    }

    handle.join().unwrap();

    Ok(())
}

fn receiver_logger(stream: TcpStream) -> Result<(), ZusiClientError> {
    let mut stream = BufReader::new(stream);

    loop {
        let msg: Message<Fahrpult> = Message::receive(&mut stream)?;

        println!("{:?}", msg);
    }
}
