import React, { useContext, useState } from "react";
import ReactDOM from "react-dom";
import { BrowserRouter as Router, Route, Switch } from "react-router-dom";
import { GlobalStyle } from "./components/GlobalStyle";

import { AppRoutes } from "./App";
import { AuthRoutes } from "./Auth";

const UserContext = React.createContext(null);

const getUser = () => {
  let user;
  const savedUser = sessionStorage.getItem("user");
  if (savedUser && savedUser !== "{}") {
    user = JSON.parse(savedUser);
  }
  return user;
}

export const useUser = () => {
  let user = useContext(UserContext);

  if (!user) {
    user = getUser();
  } else {
    const newUser = JSON.stringify(user);
    if (newUser !== "{}") {
      sessionStorage.setItem("user", newUser);
    }
  }

  return user;
};

const App = () => {
  const [user, setUser] = useState(getUser() || {});
  
  return (
    <React.StrictMode>
      <GlobalStyle />
      <Router>
        <UserContext.Provider value={{ ...(user || {}), setUser: setUser }}>
          <Switch>
            <Route path="/auth">
              <AuthRoutes setUser={setUser} />
            </Route>
            <Route path="/">
              <AppRoutes />
            </Route>
          </Switch>
        </UserContext.Provider>
      </Router>
    </React.StrictMode>
  );
};

ReactDOM.render(<App />, document.getElementById("root"));
