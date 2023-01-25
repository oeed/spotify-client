//! Because Swift-bridge's async support is rather poor, we have to split the interaction between Swift and `spotify` in to disjoint halfs.
//! The primary reason is the `spotify::Session` requires a long running async task, but

use crate::session::Session;

mod actor;
mod command;
mod request;
mod session;

pub type Error = librespot::core::Error;

use self::request::library_albums::{LibraryAlbum, LibraryArtist};

#[swift_bridge::bridge]
mod ffi {

  extern "Rust" {
    type Session;

    #[swift_bridge(init)]
    fn new(username: String, password: String) -> Session;

    #[swift_bridge(rust_name = "play_album")]
    fn playAlbum(&self, album_id: String);

    #[swift_bridge(rust_name = "next_track")]
    fn nextTrack(&self);

    fn pause(&self);

    fn play(&self);

    #[swift_bridge(rust_name = "previous_track")]
    fn previousTrack(&self);

    #[swift_bridge(rust_name = "set_track_position")]
    fn setTrackPosition(&self, milliseconds: u32);

    #[swift_bridge(rust_name = "set_volume")]
    fn setVolume(&self, volume: u16);

    #[swift_bridge(rust_name = "track_position")]
    async fn trackPosition(&self) -> u32;

    #[swift_bridge(rust_name = "library_albums")]
    async fn libraryAlbums(&self) -> LibraryAlbum;
  }

  // TODO: swift-bridge doesn't allow non-opaque types from being in Vec, change to struct types once possible
  // TODO: once swift-bridge lets us declare ffi in multiple files, move the following
  extern "Rust" {
    type LibraryAlbum;
    // type LibraryArtist;

    fn uri(&self) -> String;
  }
}
