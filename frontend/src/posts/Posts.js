import React, { useContext, useEffect } from "react";
import { useAsync } from "react-async";

import { fetchData } from "../helpers";
import { InstanceContext, CommunityContext } from "../App";

import { CreatePost } from "./CreatePost";
import { ListPosts } from "./ListPosts";

export let users = {};

const loadPosts = async ({ instance, community }) => {
  const url = new URL(`${process.env.REACT_APP_API}/posts`);
  const appendParam = (key, value) => value && url.searchParams.append(key, value);
  appendParam("host", instance);
  appendParam("community", community);
  let posts = await fetchData(url);

  const editedPosts = posts.map(async (post) => {
    try {
      if (!users[instance]) {
        users[instance] = {};
      }

      if (users[instance][post.author.id]) {
        post.user = users[instance][post.author.id];
      } else {
        try {
          post.user = await fetchData(`${process.env.REACT_APP_API}/user/${post.author.id}?host=${instance}`);
          users[instance][post.author.id] = post.user;
        } catch(e) {}
      }
    } catch (error) {}
    return post;
  });

  posts = await Promise.all([...editedPosts]);

  return posts;
};

export const Posts = () => {
  const { instance } = useContext(InstanceContext);
  const { community } = useContext(CommunityContext);

  const key = `${instance}${community}`;

  return <PostsCommunity key={key} instance={instance} community={community} />;
};

const PostsCommunity = ({ instance, community }) => {
  const { data: posts, isLoading, error, reload } = useAsync(loadPosts, { instance, community });

  return (
    <>
      <CreatePost refresh={reload} />
      <ListPosts posts={posts} isLoading={isLoading} error={error} />
    </>
  );
};
