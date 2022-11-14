use spotify_playback::session::SpotifySession;

#[tokio::main]
async fn main() {
  let player = SpotifySession::new("username", "password").await;
}
