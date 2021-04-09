import React, { useState } from "react";
import styled from "styled-components";
import { Switch, Redirect } from "react-router-dom";

import { isAuthenticated } from "./helpers";
import { Header } from "./components/Header";
import { Search } from "./components/Search";
import { ErrorHandledRoute } from "./components/ErrorHandledRoute";

import { Home } from "./Home";

const ALL_INSTANCES = "";
const INTERNAL_INSTANCE = "cs3099user-a9.host.cs.st-andrews.ac.uk";

export const InstanceContext = React.createContext(INTERNAL_INSTANCE);
export const SearchContext = React.createContext("");

export const AppRoutes = () => {
  const [instance, setInstance] = useState(INTERNAL_INSTANCE);
  const [search, setSearch] = useState("");

  if (!isAuthenticated()) return <Redirect to="/auth/login" />;

  return (
    <InstanceContext.Provider value={{ instance, setInstance, INTERNAL_INSTANCE, ALL_INSTANCES }}>
      <SearchContext.Provider value={{ search, setSearch }}>
        <Header>
          <Search />
        </Header>
        <main>
          <StyledAppRoutes>
            <Switch>
              <ErrorHandledRoute path="/">
                <Home />
              </ErrorHandledRoute>
            </Switch>
          </StyledAppRoutes>
        </main>
      </SearchContext.Provider>
    </InstanceContext.Provider>
  );
};

const StyledAppRoutes = styled.div``;
