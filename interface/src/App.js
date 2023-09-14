import { WsProvider } from "@polkadot/api";
import React, { useEffect, useState } from "react";
import { Route, BrowserRouter as Router, Routes } from "react-router-dom";

import Checkout from "./components/Checkout";
import Dashboard from "./components/Dashboard";
import Success from "./components/Success";
import { DEV_ACCOUNTS } from "./constants";

function App() {
  let [oracleRpc, setOracleRpc] = useState(null);
  let [isConnected, setIsConnected] = useState(false);

  const loadWs = async () => {
    const ws = new WsProvider("ws://127.0.0.1:3030");
    await ws.connect();
    setOracleRpc(ws);
  };

  oracleRpc?.on("connected", () => {
    console.log("connected");
    setIsConnected(true);
  });

  useEffect(() => {
    loadWs();
  }, []);

  if (isConnected) {
    return (
      <Router>
        <Routes>
          <Route path="/success" element={<Success />}></Route>
          <Route
            path="/checkout-demo"
            element={<Checkout state={{ oracleRpc }} />}
          ></Route>
          <Route
            path="/"
            element={
              <Dashboard state={{ accounts: DEV_ACCOUNTS, oracleRpc }} />
            }
          ></Route>
        </Routes>
      </Router>
    );
  }
}

export default App;
