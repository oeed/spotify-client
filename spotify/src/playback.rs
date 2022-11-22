use librespot::{
  connect::spirc::{Spirc, SpircLoadCommand},
  core::SpotifyId,
  metadata::{Album, Metadata},
  protocol::spirc::TrackRef,
};

use crate::session::LibrespotSession;

pub mod connection;

pub struct Playback {
  pub spirc: Spirc,
  session: LibrespotSession,
}

impl Playback {
  pub async fn play_album(&self, album_id: &str) -> Result<(), librespot::core::Error> {
    let album = Album::get(&self.session, &SpotifyId::from_base62(album_id)?).await?;
    let tracks = album
      .tracks()
      .map(|track_id| {
        let mut track = TrackRef::new();
        track.set_gid(Vec::from(track_id.to_raw()));
        track
      })
      .collect();

    self.spirc.activate()?;
    self.spirc.load(SpircLoadCommand {
      context_uri: format!("spotify:album:{}", album_id),
      start_playing: true,
      shuffle: false,
      repeat: false,
      playing_track_index: 0,
      tracks,
    })?;

    Ok(())
  }
}
