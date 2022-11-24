pub use librespot::core::session::Session as LibrespotSession;
use librespot::core::{authentication::Credentials, config::SessionConfig};
use librespot::{
  connect::{config::ConnectConfig, spirc::Spirc},
  core::config::DeviceType,
  playback::{
    audio_backend,
    config::{AudioFormat, PlayerConfig},
    mixer::{softmixer::SoftMixer, Mixer, MixerConfig, NoOpVolume},
    player::Player,
  },
};
use std::future::Future;

use std::pin::Pin;

use tokio::{select, sync::mpsc::UnboundedReceiver};

use crate::command::{Command, Commands};
use crate::request::{RequestResponderWrap, Requests};

pub struct Spotify {
  pub session: LibrespotSession,
  pub credentials: Credentials,
  pub playback: Spirc,
}

impl Spotify {
  async fn connect(username: &str, password: &str) -> (Spotify, impl Future<Output = ()>) {
    let session_config = SessionConfig::default();
    let credentials = Credentials::with_password(username, password);
    let session = LibrespotSession::new(session_config, None);
    let player_config = PlayerConfig::default();
    let audio_format = AudioFormat::default();
    let connect_config = ConnectConfig {
      name: String::from("Albumify"),
      device_type: DeviceType::Computer,
      initial_volume: Some(50),
      has_volume_ctrl: false, // TODO: volume control
    };
    let backend = audio_backend::find(None).unwrap();

    let player = Player::new(player_config, session.clone(), Box::new(NoOpVolume), move || {
      backend(None, audio_format)
    });

    let (spirc, spirc_task) = Spirc::new(
      connect_config,
      session.clone(),
      credentials.clone(),
      player,
      Box::new(SoftMixer::open(MixerConfig::default())),
    )
    .await
    .unwrap();

    (
      Spotify {
        playback: spirc,
        credentials,
        session,
      },
      spirc_task,
    )
  }
}

pub struct Actor<T> {
  spotify: Spotify,
  connection: Pin<Box<T>>,
  request_receiver: UnboundedReceiver<Requests>,
  command_receiver: UnboundedReceiver<Commands>,
}

impl Actor<()> {
  pub async fn new(
    username: &str,
    password: &str,
    request_receiver: UnboundedReceiver<Requests>,
    command_receiver: UnboundedReceiver<Commands>,
  ) -> Actor<impl Future<Output = ()> + Send> {
    let (spotify, connection) = Spotify::connect(&username, &password).await;

    Actor {
      request_receiver,
      command_receiver,
      connection: Box::pin(connection),
      spotify,
    }
  }
}

impl<T: Future<Output = ()> + Send> Actor<T> {
  pub async fn run(self) -> ! {
    let Actor {
      spotify,
      mut connection,
      mut request_receiver,
      mut command_receiver,
      ..
    } = self;

    loop {
      select! {
        () = &mut connection => {},

        request = request_receiver.recv() => {
          if let Some(request) = request {
            request.request_send(&spotify).await;
          }
        },

        command = command_receiver.recv() => {
          if let Some(command) = command {
            command.execute(&spotify).await.expect("Command failed");
          }
        },
      }
    }
  }
}
