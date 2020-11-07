use std::io::BufReader;
use std::net::TcpStream;
use std::thread;

use zusi_fahrpult::fahrpult::{Fahrpult, FuehrerstandsAnzeigen, NeededData};
use zusi_fahrpult::{receive_fahrpult, ZusiClientError};

fn main() -> Result<(), ZusiClientError> {
    let (mut stream, ack) = zusi_fahrpult::connect("10.211.55.3:1436")?;

    println!("{}", ack.zusi_version);

    let reader = stream.try_clone()?;
    let handle = thread::spawn(move || {
        receiver_logger(reader).unwrap();
    });

    {
        let needed_data = NeededData {
            fuehrerstands_anzeigen: Some(FuehrerstandsAnzeigen {
                anzeigen: vec![0x0001],
            }),
            fuehrerstands_bedienung: None,
            programmdaten: None,
        };
        let needed_data = Fahrpult {
            needed_data: Some(needed_data),
            ack_needed_data: None,
            data_ftd: None,
            data_prog: None,
            control: None,
        };

        zusi_fahrpult::send_fahrpult(needed_data, &mut stream)?;
    }

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
