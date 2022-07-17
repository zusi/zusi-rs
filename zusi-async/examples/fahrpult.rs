use tokio_util::codec::Decoder;
use zusi_async::ZusiProtocolDecoder;
use zusi_fahrpult::Message;

fn main() {
    let _decoder = ZusiProtocolDecoder::<Message>::new();
}
