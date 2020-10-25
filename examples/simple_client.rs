extern crate zusi;

use zusi::ser::{to_bytes, Serialize};
use zusi_protocol::Serialize;

#[derive(Serialize)]
struct HelloMsg {
    #[zusi(id = 0x0001)]
    protokoll_version: u16,
    #[zusi(id = 0x0002)]
    client_typ: u16,
    name: String,
    version: String,
}

fn main() {
    let hello = HelloMsg {
        protokoll_version: 2,
        client_typ: 2,
        name: "Fahrpult".to_string(),
        version: "1.0".to_string(),
    };

    let bts = to_bytes(&hello);
}
