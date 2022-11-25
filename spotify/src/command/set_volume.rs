use crate::{actor::Spotify, session::Session};

use super::Command;
use async_trait::async_trait;

#[derive(Debug)]
pub struct SetVolume(u16);

#[async_trait]
impl Command for SetVolume {
  async fn execute(self, spotify: &Spotify) -> Result<(), crate::Error> {
    spotify.playback.set_volume(self.0)
  }
}

impl Session {
  pub fn set_volume(&self, volume: u16) {
    self.command(SetVolume(volume))
  }
}
