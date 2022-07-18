use futures::StreamExt;
use std::error::Error;
use tokio::net::TcpStream;
use tokio_util::codec::Decoder;
use zusi::fahrpult::Fahrpult;
use zusi::Message;
use zusi_protocol::ZusiProtocolCodec;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let codec = ZusiProtocolCodec::<Message<Fahrpult, 2>>::new();
    let stream = TcpStream::connect("127.0.0.1:1435").await?;
    let (mut _sink, mut rx) = codec.framed(stream).split();

    // sink.send(Message);

    while let Some(Ok(msg)) = rx.next().await {
        println!("{:?}", msg);
    }

    Ok(())
}
