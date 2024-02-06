//! Zusi 3 TCP-Client library

#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]

pub use message::Message;

pub mod client;
pub mod message;
pub mod verbindungsaufbau;

#[cfg(feature = "fahrpult")]
pub mod fahrpult {
    pub use zusi_fahrpult::*;
}
