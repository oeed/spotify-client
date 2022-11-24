use self::pause::Pause;
use self::play_album::PlayAlbum;
use crate::actor::Spotify;
use async_trait::async_trait;
use enum_dispatch::enum_dispatch;
use std::fmt::Debug;

mod pause;
mod play_album;

#[enum_dispatch]
#[derive(Debug)]
pub enum Commands {
  PlayAlbum(PlayAlbum),
  Pause(Pause),
}

/// Commands are 'fire and forget' tasks the actor executes, but cares not of their response or when they finish.
/// As such, they can be called (in Swift) synchronously.
#[async_trait]
#[enum_dispatch(Commands)]
pub trait Command: Send + Debug {
  async fn execute(self, spotify: &Spotify) -> Result<(), crate::Error>;
}
