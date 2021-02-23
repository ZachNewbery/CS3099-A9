import React, { useState, useEffect } from "react";
import { useAsync } from "react-async";
import { fetchData } from "../helpers";
import { renderContent} from "./PostContent";
import moment from "moment";

const loadPosts = async ({ communityId }) => {
  return fetchData(`${process.env.REACT_APP_API}/posts?community=${communityId}`)
}

const Post = ({ author, children, content, created, modified, parentPost }) => {
  if (parentPost) return null;
  // author: {host: "Academoo", id: "rhona"}
  // children: []
  // community: "GeneralCowPictures"
  // content: [{text: {text: "Feature: Ban Darren↵...Coming soon to Academoo near yoo"}}]
  // created: 1613416074
  // id: "4890cc57-b378-4bc2-a5f5-94b5211d7a1d"
  // modified: 1613416074
  // parentPost: "bb6964f3-a1d3-4007-ad48-a9116b801600"
  // title: ""
  
  return (
    <div>
      <p>Author {author.id}</p>
      <p>Comments {children.length}</p>
      <p>Created {moment(created).fromNow()}</p>
      {renderContent(content)}
    </div>
  )
}

export const ListPosts = ({ communityId }) => {
  const { data: posts } = useAsync(loadPosts);
  
  return (
    <div>
      {posts.map(post => (
        <Post key={post.id} {...post} />
      ))}
    </div>
  )
};