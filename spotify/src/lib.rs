#![warn(rust_2018_idioms)]

pub use crate::playback::Playback;
pub use crate::session::Session;

pub type LibrespotResult<T> = Result<T, librespot::core::Error>;

mod playback;
mod session;
