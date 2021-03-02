import React, { useState, useEffect } from "react";
import styled from "styled-components";
import moment from "moment";

import { useAsync } from "react-async";
import { fetchData, Spinner, Error } from "../helpers";

import { renderContent } from "./PostContent";
import { posts } from "./posts";
import { Post } from "./SinglePost";

const loadPosts = async ({ host, community }) => {
  const hostParam = host ? `host=${host}&` : "";
  return fetchData(`${process.env.REACT_APP_API}/posts?${hostParam}community=${community}`);
};

const StyledPosts = styled.div``;

export const ListPosts = ({ host, community }) => {
  const { data: posts, isLoading, error } = useAsync(loadPosts, { host, community });

  if (isLoading) return <Spinner />;
  if (error) return <Error message={error} />;

  return (
    <StyledPosts>
      {posts.map((post) => (
        <Post key={post.id} {...post} />
      ))}
    </StyledPosts>
  );
};
