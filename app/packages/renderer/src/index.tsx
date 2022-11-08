import { createRoot } from "react-dom/client";
import * as React from "react";
import { Header } from "./header/header";
import { ThemeStyles } from "./theme";

const root = createRoot(document.getElementById("root") as HTMLDivElement);
root.render(
  <>
    <ThemeStyles />
    <Header />
  </>
);
