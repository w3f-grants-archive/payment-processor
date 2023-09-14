import React from "react";
import ReactDOM from "react-dom";
import { Route, BrowserRouter as Router, Switch } from "react-router-dom";

import Canceled from "./components/Cancel";
import Checkout from "./components/Checkout";
import Success from "./components/Success";

import "react-credit-cards/es/styles-compiled.css";
import "./styles.css";

function App() {
  return (
    <Router>
      <Switch>
        <Route path="/success.html">
          <Success />
        </Route>
        <Route path="/canceled.html">
          <Canceled />
        </Route>
        <Route path="/">
          <Checkout />
        </Route>
      </Switch>
    </Router>
  );
}

ReactDOM.render(<App />, document.getElementById("root"));
