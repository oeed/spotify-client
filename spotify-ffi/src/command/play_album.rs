use crate::{actor::Spotify, session::Session};
use librespot::{
  connect::spirc::SpircLoadCommand,
  core::SpotifyId,
  metadata::{Album, Metadata},
  protocol::spirc::TrackRef,
};

use super::Command;
use async_trait::async_trait;

#[derive(Debug)]
pub struct PlayAlbum {
  pub album_uri: String,
}

#[async_trait]
impl Command for PlayAlbum {
  async fn execute(self, spotify: &Spotify) -> Result<(), crate::Error> {
    // spotify.playback.play_album(&self.album_id).await.unwrap();
    let id = SpotifyId::from_uri(&self.album_uri)?;
    let album = Album::get(&spotify.session, &id).await?;
    let tracks = album
      .tracks()
      .map(|track_id| {
        let mut track = TrackRef::new();
        track.set_gid(Vec::from(track_id.to_raw()));
        track
      })
      .collect();

    spotify.playback.activate()?;

    spotify.playback.load(SpircLoadCommand {
      context_uri: format!("{}", self.album_uri),
      start_playing: true,
      shuffle: false,
      repeat: false,
      playing_track_index: 0,
      tracks,
    })?;
    Ok(())
  }
}

impl Session {
  pub fn play_album(&self, album_uri: String) {
    self.command(PlayAlbum { album_uri })
  }
}
