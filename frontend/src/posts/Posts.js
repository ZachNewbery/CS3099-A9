import React, { useContext, useEffect } from "react";
import { useAsync } from "react-async";

import { fetchData } from "../helpers";
import { InstanceContext } from "../App";
import { CommunityContext } from "../Home";

import { CreatePost } from "./CreatePost";
import { ListPosts } from "./ListPosts";

const loadPosts = async ({ instance, community }) => {
  const url = new URL(`${process.env.REACT_APP_API}/posts`);
  const appendParam = (key, value) => value && url.searchParams.append(key, value);
  appendParam("host", instance);
  appendParam("community", community);
  return fetchData(url);
};

export const Posts = () => {
  const { instance } = useContext(InstanceContext);
  const { community } = useContext(CommunityContext);
  
  const key = `${instance}${community}`;
  
  return <PostsCommunity key={key} instance={instance} community={community} />
}

const PostsCommunity = ({ instance, community }) => {
  const { data: posts, isLoading, error, reload } = useAsync(loadPosts, { instance, community });
  
  return (
    <>
      <CreatePost refresh={reload} />
      <ListPosts posts={posts} isLoading={isLoading} error={error} />
    </>
  )
}