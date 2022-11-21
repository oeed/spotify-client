use librespot::{
  connect::spirc::{Spirc, SpircLoadCommand},
  core::SpotifyId,
  metadata::{Album, Metadata},
  protocol::spirc::TrackRef,
};

use crate::session::Session;

pub mod connection;

pub struct Playback<'a> {
  pub spirc: Spirc,
  session: &'a Session,
}

impl Playback<'_> {
  pub async fn play_context(&self, context_uri: String) -> Result<(), librespot::core::Error> {
    let album = Album::get(&self.session.session, &SpotifyId::from_uri(&context_uri)?).await?;
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
      context_uri,
      start_playing: true,
      shuffle: false,
      repeat: false,
      playing_track_index: 0,
      tracks,
    })?;

    Ok(())
  }
}
