use tokio::{
  runtime::Runtime,
  sync::mpsc::{self, UnboundedSender},
  task::JoinHandle,
};

use crate::{
  actor::Actor,
  command::{CommandResponder, Commands, PlayAlbum},
};

pub struct Session {
  sender: UnboundedSender<Commands>,
  /// The session must own the tokio async runtime so that async code may execute in the background
  /// as swift-bridge's async runtime does not appear to always be running, or at least when we need it.
  _runtime: Runtime,
  /// As with the runtime, we need to hold on to the handle that's running the background tasks
  _runner: JoinHandle<()>,
}

impl Session {
  pub fn new(username: String, password: String) -> Session {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    // we have to host our own runtime beacuse the async runtime must last the entire duration of the app
    let runtime = Runtime::new().unwrap();
    let (sender, reciever) = mpsc::unbounded_channel();
    Session {
      sender,
      _runner: runtime.spawn(async move { Actor::new(&username, &password, reciever).await.run().await }),
      _runtime: runtime,
    }
  }

  pub async fn play_album(&mut self, album_id: String) {
    let (command, responder) = CommandResponder::new(PlayAlbum { album_id });

    let _ = self.sender.send(Commands::PlayAlbum(command));
    let v = responder.await.expect("Actor task has been killed");
  }
}
