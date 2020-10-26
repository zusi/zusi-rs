use std::io;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO Error")]
    Io {
        #[from]
        source: io::Error,
    },
}
