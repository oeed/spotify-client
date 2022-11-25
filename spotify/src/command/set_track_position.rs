use crate::{actor::Spotify, session::Session};

use super::Command;
use async_trait::async_trait;

type Milliseconds = u32;

#[derive(Debug)]
pub struct SetTrackPosition(Milliseconds);

#[async_trait]
impl Command for SetTrackPosition {
  async fn execute(self, spotify: &Spotify) -> Result<(), crate::Error> {
    spotify.playback.set_position_ms(self.0)
  }
}

impl Session {
  pub fn set_track_position(&self, milliseconds: Milliseconds) {
    self.command(SetTrackPosition(milliseconds))
  }
}
