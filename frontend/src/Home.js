import React, { useState, useContext, useEffect } from "react";
import styled from "styled-components";
import { Switch, Route, useLocation } from "react-router-dom";

import { InstanceContext, CommunityContext } from "./App";
import { Spinner } from "./helpers";
import { ErrorHandledRoute } from "./components/ErrorHandledRoute";
import { Posts, SinglePost } from "./posts";
import { Communities } from "./communities/Communities";

const StyledContainer = styled.div`
  display: flex;
  margin: 2rem 0;

  & > .communities-container {
    min-width: 15rem;
    max-width: 15rem;
    min-height: 20rem;
    height: 100%;
    margin-right: 1rem;
    margin-bottom: 1rem;
  }

  & > .posts-container {
    min-width: 35rem;
    max-width: 35rem;
  }
`;

export const Home = () => {
  const { community, communities } = useContext(CommunityContext);

  const location = useLocation();
  
  return (
    <StyledContainer>
      <div className="communities-container">
        <Communities />
      </div>
      <div className="posts-container">
        {community ? (
          <Switch>
            <ErrorHandledRoute path="/post/:postId">
              <SinglePost key={location.pathname} />
            </ErrorHandledRoute>
            <Route path="/">
              <Posts />
            </Route>
          </Switch>
        ) : communities ? (
          <h3 style={{ textAlign: "center", margin: "2rem 0" }}>No community selected</h3>
        ) : (
          <Spinner />
        )}
      </div>
    </StyledContainer>
  );
};
