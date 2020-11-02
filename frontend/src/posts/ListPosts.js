import React from "react";
import styled from "styled-components";
import { useAsync } from "react-async";
import { fetchData } from "../helpers";
import { Post } from "./SinglePost";

const loadPosts = async () => {
  return fetchData(`${process.env.REACT_APP_API_URL}/posts`);
}

const StyledContainer = styled.div`
  margin: auto;
  width: 500px;
  padding: 1.5em 0;
`;

export const ListPosts = () => {
  const { data, isLoading } = useAsync(loadPosts);

  if (isLoading) {
    return <h1>Loading</h1>
  }

  return (
    <StyledContainer>
      {data.map(post => <Post key={post.id} {...post} />)}
    </StyledContainer>
  )
}