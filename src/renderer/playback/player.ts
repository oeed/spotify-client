export class SpotifyPlayer {
  player: Spotify.Player;

  constructor(player: Spotify.Player) {
    this.player = player;

    // Error handling
    player.addListener("initialization_error", (err) => {
      console.error(err);
    });
    player.addListener("authentication_error", ({ message }) => {
      console.error(message);
    });
    player.addListener("account_error", ({ message }) => {
      console.error(message);
    });
    player.addListener("playback_error", ({ message }) => {
      console.error(message);
    });

    // Playback status updates
    player.addListener("player_state_changed", (state) => {
      console.log(state);
    });

    // Ready
    player.addListener("ready", ({ device_id }) => {
      console.log("Ready with Device ID", device_id);
    });

    // Not Ready
    player.addListener("not_ready", ({ device_id }) => {
      console.log("Device ID has gone offline", device_id);
    });

    // Connect to the player!
    player.connect();
  }

  static async initialise(): Promise<SpotifyPlayer> {
    return new Promise<SpotifyPlayer>((resolve, reject) => {
      window.onSpotifyWebPlaybackSDKReady = () => {
        const player = new Spotify.Player({
          getOAuthToken: (callback) =>
            callback(
            ),
          name: "Spotify Client",
        });
        resolve(new SpotifyPlayer(player));
      };
      this.loadScript().catch(reject);
    });
  }

  static loadScript(): Promise<any> {
    return new Promise<void>((resolve, reject) => {
      const scriptTag = document.getElementById("spotify-player");

      if (!scriptTag) {
        const script = document.createElement("script");

        script.id = "spotify-player";
        script.type = "text/javascript";
        script.async = false;
        script.defer = true;
        script.src = "https://sdk.scdn.co/spotify-player.js";
        script.onload = () => resolve();
        script.onerror = (error: any) =>
          reject(
            new Error(`Failed to load Spotify player API: ${error.message}`)
          );

        document.head.appendChild(script);
      } else {
        resolve();
      }
    });
  }
}
