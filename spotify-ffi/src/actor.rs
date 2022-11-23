use std::pin::Pin;

use futures::Future;
use tokio::{select, sync::mpsc::UnboundedReceiver};

use crate::command::{Command, Commands};

pub struct Spotify {
  pub _session: spotify::Session,
  pub playback: spotify::Playback,
}

pub struct Actor<T> {
  spotify: Spotify,
  connection: Pin<Box<T>>,
  receiver: UnboundedReceiver<Commands>,
}

impl Actor<()> {
  pub async fn new(
    username: &str,
    password: &str,
    receiver: UnboundedReceiver<Commands>,
  ) -> Actor<impl Future<Output = ()> + Send> {
    let session = spotify::Session::new(&username, &password);
    let (playback, connection) = spotify::Playback::connect(&session).await;

    Actor {
      receiver,
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
      mut receiver,
      ..
    } = self;

    loop {
      select! {
        () = &mut connection => {},

        command = receiver.recv() => {
          if let Some(command) = command {
            match command {
              Commands::PlayAlbum(responder) => {
                let response = responder.command.execute(&spotify).await;
                // we can ignore any errors while sending: https://ryhl.io/blog/actors-with-tokio/
                let _ = responder.respond_to.send(response);
              }
            }
          }
        },
      }
    }
  }
}
