import React, { useState, useEffect } from "react";
import styled from "styled-components";
import { Switch, Route } from "react-router-dom";

import { ErrorHandledRoute } from "./components/ErrorHandledRoute";
import { CreatePost, ListPosts, SinglePost } from "./posts";
import { ListCommunities } from "./communities/ListCommunities";
import { SingleCommunity } from "./communities/SingleCommunity";

const StyledContainer = styled.div`
  display: flex;
  margin: 2rem 0;

  & > .communities-container {
    width: 15rem;
    min-height: 20rem;
    max-height: 30rem;
    margin-right: 1rem;
  }

  & > .posts-container {
    width: 35rem;
  }
`;

export const Home = ({ search, host }) => {
  const [community, setCommunity] = useState(null);
  const [i, setI] = useState(0);

  const reload = () => setI(i => i + 1);

  console.log(community);
  
  return (
    <StyledContainer>
      <div className="communities-container">
        {community && <SingleCommunity id={community} host={host} />}
        <ListCommunities setCommunity={setCommunity} community={community} host={host} />
      </div>
      <div className="posts-container">
        <Switch>
          <ErrorHandledRoute path="/post/:postId">
            <SinglePost />
          </ErrorHandledRoute>
          <Route path="/">
            <CreatePost community={community} host={host} refresh={reload} />
            {community && <ListPosts key={`${community}${i}`} community={community} host={host} search={search} />}
          </Route>
        </Switch>
      </div>
    </StyledContainer>
  );
};
