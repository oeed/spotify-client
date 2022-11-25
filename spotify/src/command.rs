use crate::actor::Spotify;
use async_trait::async_trait;
use enum_dispatch::enum_dispatch;
use std::fmt::Debug;

mod next_track;
mod pause;
mod play;
mod play_album;
mod previous_track;
mod set_track_position;
mod set_volume;

#[enum_dispatch]
#[derive(Debug)]
pub enum Commands {
  NextTrack(next_track::NextTrack),
  Pause(pause::Pause),
  Play(play::Play),
  PlayAlbum(play_album::PlayAlbum),
  PreviousTrack(previous_track::PreviousTrack),
  SetPosition(set_track_position::SetTrackPosition),
  SetVolume(set_volume::SetVolume),
}

/// Commands are 'fire and forget' tasks the actor executes, but cares not of their response or when they finish.
/// As such, they can be called (in Swift) synchronously.
#[async_trait]
#[enum_dispatch(Commands)]
pub trait Command: Send + Debug {
  async fn execute(self, spotify: &Spotify) -> Result<(), crate::Error>;
}
