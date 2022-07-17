use std::marker::PhantomData;

use bytes::BytesMut;
use tokio_util::codec::Decoder;
use zusi_protocol::RootMessage;

mod dto;

pub struct ZusiProtocolDecoder<T>
where
    T: RootMessage,
{
    phantom: PhantomData<*const T>,
}

impl<T: RootMessage> ZusiProtocolDecoder<T> {
    pub fn new() -> Self {
        Self { phantom: PhantomData }
    }
}

impl<T: RootMessage> Decoder for ZusiProtocolDecoder<T> {
    type Item = T;

    type Error = std::io::Error;

    fn decode(&mut self, _src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
