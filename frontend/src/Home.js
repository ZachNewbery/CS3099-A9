import React from "react";
import styled from "styled-components";
import { Redirect, Link, Switch, Route } from "react-router-dom";
import { isAuthenticated } from "./helpers";
import { ListPosts, SinglePost, CreatePost } from "./posts";

const StyledContainer = styled.div`
  width: 100%;
`;

const StyledHeader = styled.div`
  width: 100%;
  padding: 5px 0;
  border-bottom: 1px solid lightgray;
  background: white;
  box-sizing: border-box;
  display: flex;
  justify-content: center;
  align-items: center;
  & > div {
    width: 500px;
    margin: auto;
    display: flex;
    justify-content: space-between;
    & > a {
      padding: 5px 10px;
    }
  }
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
      <div>
        <Link to="/logout">Logout</Link>
        <Link to="/">Home</Link>
        <Link to="/create-post">Create Post</Link>
      </div>
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
            <SinglePost />
          </Route>
          <Route path="/create-post">
            <CreatePost />
          </Route>
          <Route path="/">
            <ListPosts />
          </Route>
        </Switch>
      </StyledContent>
    </StyledContainer>
  )
}