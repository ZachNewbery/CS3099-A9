import React, { useState } from "react";
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

  const reload = (communityId = null) => {
    console.log({ community, communityId });
    setCommunity(communityId);
    setI(i => i + 1);
  }

  return (
    <StyledContainer>
      <div className="communities-container">
        {community && <SingleCommunity key={community} id={community} host={host} refresh={reload} />}
        <ListCommunities key={i} setCommunity={setCommunity} community={community} host={host} refresh={reload} />
      </div>
      <div className="posts-container">
        <Switch>
          <ErrorHandledRoute path="/post/:postId">
            <SinglePost community={community} setCommunity={setCommunity} />
          </ErrorHandledRoute>
          <Route path="/">
            <CreatePost key={community} community={community} host={host} refresh={() => reload(community)} />
            {community && <ListPosts key={`${community}${i}`} community={community} host={host} search={search} />}
          </Route>
        </Switch>
      </div>
    </StyledContainer>
  );
};
