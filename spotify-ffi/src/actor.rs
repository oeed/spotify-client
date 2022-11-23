use std::pin::Pin;

use futures::Future;
use tokio::{select, sync::mpsc::UnboundedReceiver};

use crate::command::{Command, Commands};
use crate::request::{RequestResponderWrap, Requests};

pub struct Spotify {
  pub _session: spotify::Session,
  pub playback: spotify::Playback,
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
    let session = spotify::Session::new(&username, &password);
    let (playback, connection) = spotify::Playback::connect(&session).await;

    Actor {
      request_receiver,
      command_receiver,
      connection: Box::pin(connection),
      spotify: Spotify {
        _session: session,
        playback,
      },
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
            command.execute(&spotify).await;
          }
        },
      }
    }
  }
}
