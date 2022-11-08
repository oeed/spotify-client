import React, { FunctionComponent } from "react";
import { colours, layout } from "../theme";
import { css } from "@emotion/react";

interface NowPlayingProps {}

export const NowPlaying: FunctionComponent<NowPlayingProps> = () => (
  <div
    css={css`
      display: flex;
      align-items: center;
      width: ${layout.header.nowPlaying.width};
      padding: ${layout.header.nowPlaying.padding.vertical}
        ${layout.header.nowPlaying.padding.horizontal};
      background-color: ${colours.header.nowPlaying};
      border-left: ${layout.splitter.primary};
      border-right: ${layout.splitter.primary};
    `}
  >
    <AlbumArtwork />
    <div>
      <h5>Magma</h5>
      <h6>Ice, Death, Planets, Lungs, Mushrooms and Lava</h6>
    </div>
  </div>
);

interface AlbumArtworkProps {}

const AlbumArtwork: FunctionComponent<AlbumArtworkProps> = () => (
  <button
    css={css`
      width: calc(
        ${layout.header.height} - 2 *
          ${layout.header.nowPlaying.padding.vertical}
      );
      height: calc(
        ${layout.header.height} - 2 *
          ${layout.header.nowPlaying.padding.vertical}
      );
      outline: none;
      border: none;
      margin-right: ${layout.header.nowPlaying.padding.vertical};
      background-size: cover;
      border-radius: ${layout.album.small.radius};
      box-shadow: ${colours.album.small.shadow};
      -webkit-app-region: no-drag;
    `}
  />
);
