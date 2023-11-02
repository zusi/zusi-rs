use futures::StreamExt;
use std::error::Error;
use tokio::net::TcpStream;
use tokio_util::codec::Decoder;
use zusi::client::async_codec::ZusiProtocolCodec;
use zusi_fahrpult::Fahrpult;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let codec = ZusiProtocolCodec::<Fahrpult>::new();
    let stream = TcpStream::connect("127.0.0.1:1435").await?;
    let (_sink, mut rx) = codec.framed(stream).split();

    // sink.send(Message);

    while let Some(Ok(msg)) = rx.next().await {
        println!("{:?}", msg);
    }

    Ok(())
}
