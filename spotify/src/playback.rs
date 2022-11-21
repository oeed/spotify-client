use librespot::{
  connect::{
    config::ConnectConfig,
    spirc::{Spirc, SpircLoadCommand},
  },
  core::{authentication::Credentials, config::SessionConfig, session::Session},
  playback::{
    audio_backend,
    config::{AudioFormat, PlayerConfig},
    mixer::{softmixer::SoftMixer, Mixer, MixerConfig, NoOpVolume},
    player::Player,
  },
  protocol::spirc::TrackRef,
};
use std::future::Future;

pub mod connection;

pub struct Playback {
  pub spirc: Spirc,
}

impl Playback {
  pub fn play_context(&self, context_uri: String) -> Result<(), librespot::core::Error> {
    self.spirc.activate()?;

    self.spirc.load(SpircLoadCommand {
      context_uri,
      start_playing: true,
      shuffle: false,
      repeat: false,
      playing_track_index: 0,
      tracks: [
        [246, 75, 3, 149, 230, 100, 65, 22, 165, 244, 244, 36, 6, 174, 28, 144],
        [36, 78, 45, 249, 108, 77, 69, 144, 140, 156, 136, 92, 151, 11, 25, 205],
        [75, 254, 83, 110, 93, 161, 72, 204, 188, 190, 71, 93, 74, 153, 169, 176],
        [161, 247, 183, 239, 76, 20, 78, 82, 181, 249, 33, 107, 145, 9, 142, 189],
        [155, 203, 88, 196, 44, 120, 78, 64, 145, 130, 212, 54, 6, 175, 27, 207],
        [232, 87, 220, 191, 198, 32, 76, 236, 169, 6, 182, 155, 211, 191, 28, 236],
        [
          179, 230, 217, 247, 6, 150, 67, 63, 139, 28, 174, 245, 101, 173, 212, 221,
        ],
        [
          129, 148, 175, 113, 172, 209, 71, 250, 147, 213, 166, 211, 208, 196, 157, 218,
        ],
        [
          183, 89, 206, 153, 158, 127, 66, 48, 139, 191, 214, 166, 66, 216, 75, 170,
        ],
        [57, 124, 70, 176, 111, 199, 64, 138, 187, 203, 87, 87, 123, 189, 75, 73],
        [254, 26, 85, 102, 54, 233, 79, 206, 142, 97, 84, 83, 244, 111, 48, 51],
        [
          150, 193, 60, 194, 174, 212, 64, 250, 149, 96, 137, 107, 43, 182, 87, 185,
        ],
        [
          106, 234, 165, 111, 20, 114, 65, 55, 144, 32, 215, 176, 164, 133, 230, 40,
        ],
      ]
      .iter()
      .map(|gid| {
        let mut track = TrackRef::new();
        track.set_gid(gid.to_vec());
        track
      })
      .collect(),
    })?;

    Ok(())
  }
}
