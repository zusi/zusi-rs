//! Zusi 3 TCP-Client library

#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]

pub use message::Message;
pub mod client;
pub mod message;

/// Protocol implementation for handshake and initial connection setup.
pub mod verbindungsaufbau;

#[cfg(feature = "fahrpult")]
#[cfg_attr(docsrs, doc(cfg(feature = "fahrpult")))]
/// Protocol implementation for Fahrpult mode.
pub mod fahrpult {
    pub use zusi_fahrpult::*;
}
