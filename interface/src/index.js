import { createRoot } from "react-dom/client";
import App from "./App";

import "react-credit-cards/es/styles-compiled.css";
import "semantic-ui-css/semantic.min.css";
import "./styles.css";

const root = createRoot(document.getElementById("root"));
root.render(<App tab="home" />);
