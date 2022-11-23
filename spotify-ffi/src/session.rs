use std::pin::Pin;

use futures::Future;
use tokio::{
  runtime::Runtime,
  select,
  sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
  task::JoinHandle,
};

pub struct Session {
  tx: UnboundedSender<String>,
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
    let (tx, rx) = mpsc::unbounded_channel();
    let _ = runtime.enter();
    Session {
      tx,
      _runner: runtime.spawn(async move { Runner::new(&username, &password, rx).await.run().await }),
      _runtime: runtime,
    }
  }

  pub fn play_album(&mut self, album_id: String) {
    self.tx.send(album_id).unwrap();
  }
}

struct Runner<T> {
  _session: spotify::Session,
  playback: spotify::Playback,
  connection: Pin<Box<T>>,
  rx: UnboundedReceiver<String>,
}

impl Runner<()> {
  pub async fn new(
    username: &str,
    password: &str,
    rx: UnboundedReceiver<String>,
  ) -> Runner<impl Future<Output = ()> + Send> {
    let session = spotify::Session::new(&username, &password);
    let (playback, connection) = spotify::Playback::connect(&session).await;

    Runner {
      rx,
      playback,
      connection: Box::pin(connection),
      _session: session,
    }
  }
}

impl<T: Future<Output = ()> + Send> Runner<T> {
  async fn run(self) -> ! {
    let Runner {
      playback,
      mut connection,
      mut rx,
      ..
    } = self;

    loop {
      select! {
        () = &mut connection => {},
        command = rx.recv() => {
          if let Some(command) = command {
            playback.play_album(&command).await.unwrap();
          }
        },
      }
    }
  }
}
