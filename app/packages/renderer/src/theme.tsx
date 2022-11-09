import { Global, css } from "@emotion/react";
import { FunctionComponent } from "react";
import { colour, darkLightStyles } from "./helpers/theme.helper";

export const colours = {
  background: colour("background", "white", "black"),
  header: {
    background: colour("header-background", "#fafafa", "#fafafa"),
    nowPlaying: colour("header-now-playing", "#f1f1f1", "#f1f1f1"),
  },
  splitter: {
    primary: colour(
      "splitter-primary",
      "rgba(0 0 0 / 10%)",
      "rgba(255 255 255 / 10%)"
    ),
  },
  album: {
    small: {
      shadow: colour(
        "album-small-shadow",
        "inset 0 0 0 0.5px rgb(0 0 0 / 10%), 0 1px 2px 0 rgb(0 0 0 / 20%)",
        "inset 0 0 0 0.5px rgb(0 0 0 / 10%), 0 1px 2px 0 rgb(0 0 0 / 20%)"
      ),
    },
  },
};

export const layout = {
  header: {
    height: "52px",
    nowPlaying: {
      width: "50%",
      padding: {
        vertical: "10px",
        horizontal: "15px",
      },
    },
  },
  splitter: {
    primary: `0.5px solid ${colours.splitter.primary}`,
  },
  album: {
    small: {
      radius: "3px",
    },
  },
};

// See: https://developer.apple.com/design/human-interface-guidelines/foundations/typography#specifications
export const fontStyles = {
  headline: css`
    font-size: 13px;
    line-height: 16px;
    font-weight: bold;
  `,
  subheadline: css`
    font-size: 11px;
    line-height: 14px;
    font-weight: normal;
  `,
};

const globalStyles = css`
  body {
    margin: 0;
    padding: 0;
    font-family: system-ui;
    background: ${colours.background};
    -webkit-user-select: none;
  }

  h1,
  h2,
  h3,
  h4,
  h5,
  h6,
  p {
    margin: 0;
  }

  h5 {
    ${fontStyles.headline};
  }

  h6 {
    ${fontStyles.subheadline};
  }
`;

const { darkStyles, lightStyles } = darkLightStyles(colours);
export const ThemeStyles: FunctionComponent = () => (
  <Global
    styles={css`
      ${globalStyles}

      :root {
        ${lightStyles};
      }

      @media (prefers-color-scheme: dark) {
        :root {
          ${darkStyles}
        }
      }
    `}
  />
);
