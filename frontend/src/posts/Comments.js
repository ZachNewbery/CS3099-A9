import React, { useRef, useState } from "react";
import styled from "styled-components";
import moment from "moment";
import { useAsync } from "react-async";
import { fetchData, getCurrentUser, Spinner, Error } from "../helpers";
import { StyledBlock, StyledContent, renderContent } from "./PostContent";

const loadChildPosts = async ({ children }) => {
  let posts = [];

  for (const child of children) {
    const post = await fetchData(`${process.env.REACT_APP_API}/posts/${child}`);
    posts.push(post);
  }

  return posts;
};

const createComment = async ({ postId, communityId, content }) => {
  const post = {
    parent: postId,
    content: [
      {
        text: content,
      },
    ],
    community: {
      id: communityId, // I changed this :( lol
    },
    title: "atitle",
  };

  return fetchData(`${process.env.REACT_APP_API}/posts/create`, JSON.stringify(post), "POST");
};

const Comment = ({ author, content, created, modified }) => {
  return (
    <StyledContent>
      {content.map((block, i) => (
        <StyledBlock key={i}>{renderContent(block)}</StyledBlock>
      ))}
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
        await createComment({ postId, content, communityId });
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
  const { data: comments, isLoading, error } = useAsync(loadChildPosts, { children });

  if (isLoading) return <Spinner />;
  if (error) return <Error message={error} />;

  console.log(comments);

  return (
    <StyledComments>
      {comments.map((comment) => (
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
