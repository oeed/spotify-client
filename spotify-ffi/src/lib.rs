//! Because Swift-bridge's async support is rather poor, we have to split the interaction between Swift and `spotify` in to disjoint halfs.
//! The primary reason is the `spotify::Session` requires a long running async task, but

use crate::session::Session;

mod actor;
mod command;
mod session;

#[swift_bridge::bridge]
mod ffi {
  extern "Rust" {
    type Session;

    #[swift_bridge(init)]
    fn new(username: String, password: String) -> Session;

    #[swift_bridge(rust_name = "play_album")]
    async fn playAlbum(&mut self, album_id: String);
  }
}
