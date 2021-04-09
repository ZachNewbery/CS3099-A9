import React, { useState, useContext, useEffect } from "react";
import styled from "styled-components";
import { Switch, Route } from "react-router-dom";

import { InstanceContext } from "./App";
import { Spinner } from "./helpers";
import { ErrorHandledRoute } from "./components/ErrorHandledRoute";
import { Posts, SinglePost } from "./posts";
import { Communities } from "./communities/Communities";

const StyledContainer = styled.div`
  display: flex;
  margin: 2rem 0;

  & > .communities-container {
    width: 15rem;
    min-height: 20rem;
    height: 100%;
    margin-right: 1rem;
    margin-bottom: 1rem;
  }

  & > .posts-container {
    width: 35rem;
  }
`;

export const CommunityContext = React.createContext(null);

export const Home = () => {
  const [community, setCommunity] = useState(null);
  const { instance } = useContext(InstanceContext);

  useEffect(() => {
    setCommunity(null);
  }, [instance]);
  
  return (
    <CommunityContext.Provider value={{ community, setCommunity }}>
      <StyledContainer>
        <div className="communities-container">
          <Communities />
        </div>
        {community ? (
          <div className="posts-container">
            <Switch>
              <ErrorHandledRoute path="/post/:postId">
                <SinglePost />
              </ErrorHandledRoute>
              <Route path="/">
                <Posts />
              </Route>
            </Switch>
          </div>
        ) : (
          <Spinner />
        )}
      </StyledContainer>
    </CommunityContext.Provider>
  );
};
