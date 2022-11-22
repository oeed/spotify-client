#![warn(rust_2018_idioms)]

pub use crate::playback::Playback;
pub use crate::session::Session;

mod playback;
mod session;

#[swift_bridge::bridge]
mod ffi {
  extern "Rust" {
    type Session;

    #[swift_bridge(init)]
    fn new(username: &str, password: &str) -> Session;

    fn playback(&self) -> Playback;
  }

  extern "Rust" {
    type Playback;

    #[swift_bridge(swift_name = "playAlbum")]
    async fn play_album_ffi(&self, album_id: &str);
  }
}

impl Playback {
  // swift-bridge doesn't yet support returning Results
  async fn play_album_ffi(&self, album_id: &str) {
    self.play_album(album_id).await.unwrap()
  }
}
