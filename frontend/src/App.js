import React, { useState, useEffect } from "react";
import styled from "styled-components";
import { Switch, Redirect, useLocation } from "react-router-dom";

import { isAuthenticated } from "./helpers";
import { Header } from "./components/Header";
import { Search } from "./components/Search";
import { ErrorHandledRoute } from "./components/ErrorHandledRoute";

import { Home } from "./Home";

const ALL_INSTANCES = "";
const INTERNAL_INSTANCE = "cs3099user-a9.host.cs.st-andrews.ac.uk";

export const InstanceContext = React.createContext(INTERNAL_INSTANCE);
export const SearchContext = React.createContext("");
export const CommunityContext = React.createContext(null);

export const AppRoutes = () => {
  const url = new URL(window.location.href);

  const community = url.searchParams.get("community");
  const host = url.searchParams.get("host");

  return <AppRoutesComponent initialCommunity={community} initialHost={host} />;
};

export const AppRoutesComponent = ({ initialCommunity, initialHost }) => {
  const [instance, setInstance] = useState(initialHost || INTERNAL_INSTANCE);
  const [search, setSearch] = useState("");
  const [community, setCommunity] = useState(initialCommunity || null);
  const [communities, setCommunities] = useState(null);

  const location = useLocation();

  useEffect(() => {
    const url = new URL(window.location.href);
    url.searchParams.set("community", community);
    url.searchParams.set("host", instance);
    window.history.replaceState({}, "", url);
  }, [community, instance, location]);

  if (!isAuthenticated()) return <Redirect to="/auth/login" />;

  return (
    <InstanceContext.Provider value={{ instance, setInstance, INTERNAL_INSTANCE, ALL_INSTANCES }}>
      <SearchContext.Provider value={{ search, setSearch }}>
        <CommunityContext.Provider value={{ community, setCommunity, communities, setCommunities }}>
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
        </CommunityContext.Provider>
      </SearchContext.Provider>
    </InstanceContext.Provider>
  );
};

const StyledAppRoutes = styled.div``;
