import React from "react";
import ReactDOM from "react-dom";
import { BrowserRouter as Router, Route, Switch } from "react-router-dom";
import { GlobalStyle } from "./components/GlobalStyle";

import { AppRoutes } from "./App";
import { AuthRoutes } from "./Auth";

ReactDOM.render(
  <React.StrictMode>
    <GlobalStyle />
    <Router>
      <Switch>
        <Route path="/auth">
          <AuthRoutes />
        </Route>
        <Route path="/">
          <AppRoutes />
        </Route>
      </Switch>
    </Router>
  </React.StrictMode>,
  document.getElementById("root")
);