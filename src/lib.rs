mod error;
mod ser;
use ser::Serialize;
use zusi_protocol::Serialize;

pub(self) use crate::error::{Error, Result};

#[derive(Serialize)]
struct Test {
    #[zusi(id = 0x0001)]
    protokoll_version: u16,
    #[zusi(id = 0x0002)]
    client_typ: u16,
    name: String,
    version: String,
}
