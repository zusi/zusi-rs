use std::error::Error;

use futures::{SinkExt, Stream, StreamExt};
use tokio::net::TcpStream;
use tokio_util::codec::Decoder;

use zusi::client::ZusiProtocolCodec;
use zusi::fahrpult::Fahrpult;
use zusi::Message;
use zusi_fahrpult::{FuehrerstandsAnzeigen, NeededData};
use zusi_protocol::ProtocolError;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let codec = ZusiProtocolCodec::<Fahrpult>::new();
    let stream = TcpStream::connect("127.0.0.1:1435").await?;
    let (mut sink, rx) = codec.framed(stream).split();

    let handle = tokio::task::spawn(async move {
        receiver_logger(rx).await;
    });

    {
        let needed_data = NeededData {
            fuehrerstands_anzeigen: Some(FuehrerstandsAnzeigen { anzeigen: vec![0x0001] }),
            ..Default::default()
        };
        let needed_data = Fahrpult { needed_data: Some(needed_data), ..Default::default() };
        sink.send(needed_data.into()).await?;
    }

    handle.await.unwrap();

    Ok(())
}

async fn receiver_logger(
    mut rx: impl Stream<Item = Result<Message<Fahrpult>, ProtocolError>> + Unpin,
) {
    while let Some(Ok(msg)) = rx.next().await {
        println!("{:?}", msg);
    }
}
