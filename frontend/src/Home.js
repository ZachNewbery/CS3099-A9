import React from "react";
import styled from "styled-components";
import { Redirect, Link, Switch, Route } from "react-router-dom";
import { isAuthenticated } from "./helpers";
import { Posts, SinglePosts } from "./Posts";

const StyledContainer = styled.div`
  width: 100%;
`;

const StyledHeader = styled.div`
  width: 100%;
  padding: 10px;
  border-bottom: 1px solid lightgray;
  background: white;
  box-sizing: border-box;
`;

const StyledContent = styled.div`
  width: 100%;
  display: flex;
  align-items: center;
  flex: 1;
  background: #ffffff;
`;

const Header = () => {
  return (
    <StyledHeader>
      <Link to="/logout">Logout</Link>
    </StyledHeader>
  )
}

export const Home = () => {
  if (!isAuthenticated()) return <Redirect to='/login' />;

  return (
    <StyledContainer>
      <Header />
      <StyledContent>
        <Switch>
          <Route path="/post/:postId">
            <SinglePosts />
          </Route>
          <Route path="/">
            <Posts />
          </Route>
        </Switch>
      </StyledContent>
    </StyledContainer>
  )
}