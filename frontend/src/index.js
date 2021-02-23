import React from "react";
import ReactDOM from "react-dom";
import { BrowserRouter as Router, Route, Switch } from "react-router-dom";

import { Home } from "./Home";
import { Login, StyledAuth } from "./Login";
import { Logout } from "./Logout";
import { Registration } from "./Registration";
import { GlobalStyle } from "./components/GlobalStyle";
import { Header } from "./components/Header";
import { ErrorHandledRoute } from "./components/ErrorHandledRoute";

ReactDOM.render(
  <React.StrictMode>
    <GlobalStyle />
    <Router>
      <Header />
      <main>
        <Switch>
          <ErrorHandledRoute path="/registration">
            <StyledAuth>
              <Registration />
            </StyledAuth>
          </ErrorHandledRoute>
          <ErrorHandledRoute path="/login">
            <StyledAuth>
              <Login />
            </StyledAuth>
          </ErrorHandledRoute>
          <ErrorHandledRoute path="/logout">
            <Logout />
          </ErrorHandledRoute>
          <ErrorHandledRoute path="/">
            <Home />
          </ErrorHandledRoute>
        </Switch>
      </main>
    </Router>
  </React.StrictMode>,
  document.getElementById("root")
);
