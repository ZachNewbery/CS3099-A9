import React, { useRef, useState } from "react";
import styled from "styled-components";
import moment from "moment";
import { useAsync } from "react-async";
import { fetchData, getCurrentUser } from "../helpers";
import { StyledBlock, StyledContent, renderContent } from "./PostContent";

const loadChildPosts = async ({ children, postId }) => {
  let posts = [];

  for (const child of children) {
    const post = await fetchData(`${process.env.REACT_APP_API}/posts/${postId}`);
    posts.push(post);
  }
  
  return posts;
};

const createComment = async ({ postId, communityId, content }) => {
  // author: {id: "Fraser", host: "Academoo"}
  // community: "GeneralCowPictures"
  // content: [{text: {text: "bruh"}}]
  // parentPost: "bb6964f3-a1d3-4007-ad48-a9116b801600"
  // title: ""

  const post = {
    parentPost: postId,
    content: [
      {
        text: {
          text: content
        },
      },
    ],
    community: communityId,
    title: "",
    author: getCurrentUser()
  };

  return fetchData(
    `${process.env.REACT_APP_API}/posts`,
    JSON.stringify(post),
    "POST"
  );
};

const Comment = ({ author, content, created, modified }) => {
  // author: {host: "Academoo", id: "darren"}
  // children: []
  // community: "GeneralCowPictures"
  // content: [{text: {text: "no! ban me u wont"}}]
  // 0: {text: {text: "no! ban me u wont"}}
  // created: 1613414547
  // id: "3ce1e327-7ca8-4569-b24f-dc57632dee4d"
  // modified: 1613414547
  // parentPost: "bb6964f3-a1d3-4007-ad48-a9116b801600"
  // title: ""
  
  return (
    <StyledContent>
          {renderContent(content)}
      <hr />
      <div className="header">
        <p className="user" title={author.id}>
          {author.id}
        </p>
        <div className="date-time">
          <p className="date" style={{ marginRight: "0.5em" }}>
            {moment(created).fromNow()}
          </p>
        </div>
      </div>
    </StyledContent>
  );
};

export const CreateComment = ({ postId, communityId, refresh }) => {
  const contentRef = useRef(null);
  const [errors, setErrors] = useState({});

  const handleSubmit = async (e) => {
    e.preventDefault();
    let currentErrors = {};

    const content = contentRef.current.value;

    if (content.length < 5) {
      currentErrors.content = "Comment is too short";
    }

    if (Object.keys(currentErrors).length === 0) {
      try {
        await createComment({ postId, content, communityId })
        return refresh();
      } catch (error) {
        currentErrors.content = error.message; // TODO: see how they're passing errors
      }
    }

    setErrors(currentErrors);
  };

  return (
    <StyledContent style={{ background: "#f8f9f9", padding: 0 }}>
      <StyledCreateComment>
        <input ref={contentRef} placeholder="Enter comment" />
        <p>{errors.content}</p>
        <button onClick={handleSubmit}>Post</button>
      </StyledCreateComment>
    </StyledContent>
  );
};

export const Comments = ({ children }) => {
  const { data, isLoading } = useAsync(loadChildPosts, { children });

  if (isLoading) {
    return <h1>Loading Comments</h1>;
  }

  return (
    <StyledComments>
      {data.map((comment) => (
        <Comment key={comment.id} {...comment} />
      ))}
    </StyledComments>
  );
};


const StyledComments = styled.div`
  margin: 0;
  font-size: 0.8em;
  & > div {
    padding: 5px 10px;
    cursor: auto;
    background-color: #f8f9f9;
  }
  .user {
    flex: 1;
    font-weight: bold;
    margin: 0;
    color: #676767;
  }
  .date-time {
    font-size: 0.9em;
    flex-flow: row nowrap;
  }
  img {
    height: 100px !important;
    width: auto !important;
  }
`;

const StyledCreateComment = styled.div`
  padding: 1em;
  cursor: auto;
  display: flex;
  flex-flow: column nowrap;
  align-items: flex-start;
  & > input {
    border: 1px solid lightgray;
    width: 100%;
    border-radius: 0.3em;
    padding: 10px;
    color: inherit;
    font: inherit;
    font-size: 1em;
    box-sizing: border-box;
  }
  & > p {
    margin: 0.5em 0;
  }
  & > button {
    align-self: flex-end;
    border: 1px solid lightgray;
    background: #e5e5e5;
    border-radius: 0.3em;
    cursor: pointer;
    padding: 0.2em 1em;
  }
`;
