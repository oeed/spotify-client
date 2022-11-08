import { createRoot } from "react-dom/client";
import * as React from "react";

const root = createRoot(document.getElementById("root") as HTMLDivElement);
const element = <h1>Hello, world</h1>;
root.render(element);
