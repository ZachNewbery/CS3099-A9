import React from 'react';
import ReactDOM from 'react-dom';
import { BrowserRouter as Router, Route, Switch } from 'react-router-dom'
import { Home } from './Home';
import { Login } from './Login';
import { Logout } from './Logout';
import { Registration } from './Registration';

ReactDOM.render(
  <React.StrictMode>
    <Router>
      <Switch>
        <Route path='/registration'>
          <Registration />
        </Route>
        <Route path="/login">
          <Login />
        </Route>
        <Route path="/logout">
          <Logout />
        </Route>
        <Route path="/">
          <Home />
        </Route>
      </Switch>
    </Router>
  </React.StrictMode>,
  document.getElementById('root')
);
