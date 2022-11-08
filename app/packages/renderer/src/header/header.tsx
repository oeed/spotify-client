import React, { FunctionComponent } from "react";
import { css } from "@emotion/react";
import { layout, colours } from "../theme";
import { NowPlaying } from "./now-playing";

interface HeaderProps {}

export const Header: FunctionComponent<HeaderProps> = ({}) => (
  <header
    css={css`
      position: fixed;
      top: 0;
      height: ${layout.header.height};
      width: 100vw;
      display: flex;
      justify-content: center;
      background-color: ${colours.header.background};
      border-bottom: var(--primary-splitter);
      -webkit-app-region: drag;
    `}
  >
    <NowPlaying />
  </header>
);
