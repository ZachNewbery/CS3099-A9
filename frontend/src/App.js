import React, { useState } from "react";
import styled from "styled-components";
import { Switch, Redirect } from "react-router-dom";

import { isAuthenticated } from "./helpers";
import { Header } from "./components/Header";
import { Search } from "./components/Search";
import { ErrorHandledRoute } from "./components/ErrorHandledRoute";

import { Home } from "./Home";

export const AppRoutes = () => {
  const [filters, setFilters] = useState({ search: null, host: null })
  
  if (!isAuthenticated()) return <Redirect to="/auth/login" />;

  return (
    <>
      <Header>
        <Search {...filters} setFilters={setFilters} />
      </Header>
      <main>
        <StyledAppRoutes>
          <Switch>
            <ErrorHandledRoute path="/">
              <Home {...filters} />
            </ErrorHandledRoute>
          </Switch>
        </StyledAppRoutes>
      </main>
    </>
  );
};

const StyledAppRoutes = styled.div``;
