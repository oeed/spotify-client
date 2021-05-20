import React, { FunctionComponent, useEffect, useState } from "react";
import { SpotifyPlayer } from "./player";

export const SpotifyPlayback: FunctionComponent = () => {
  const [player, setPlayer] = useState<SpotifyPlayer | null>(null);
  useEffect(() => {
    SpotifyPlayer.initialise().then(setPlayer);
  });

  return <>playback</>;
};

export function loadSpotifyPlayer(): Promise<any> {
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
