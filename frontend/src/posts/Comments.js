import React, { useRef, useState } from "react";
import styled from "styled-components";
import moment from "moment";
import { useAsync } from "react-async";
import { fetchData, Spinner, Error, colors } from "../helpers";
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
      id: communityId,
    },
    title: "atitle",
  };

  return fetchData(`${process.env.REACT_APP_API}/posts/create`, JSON.stringify(post), "POST");
};

const StyledComments = styled.div`
  background: ${colors.pageBackground};
  padding: 1rem 0;
  & > .comment {
    padding: 0 1rem;
    &:not(:first-child) {
      padding-top: 1rem;
    }
    & > .main {
      display: flex;
      align-items: center;
      & > .profile > img {
        height: 2.5rem;
        width: 2.5rem;
        border-radius: 1.5rem;
        margin-right: 0.5rem;
      }
      & > .content {
        width: 100%;
        background: white;
        border: 1px solid ${colors.mediumLightGray};
        border-radius: 0.6rem;
        padding: 0 1rem;
      }
    }
    & > .footer {
      margin: 0.25rem;
      margin-left: 3.5rem;
      & > p {
        margin: 0;
        font-size: 0.75rem;
      }
    }
  }
`;

const StyledCreateComment = styled.div`
  padding: 0.8rem 0;
  margin: 0 0.8rem;
  & > input {
    border: 1px solid ${colors.veryLightGray};
    width: 100%;
    border-radius: 0.6em;
    padding: 0.5rem 0.7rem;
    color: inherit;
    font: inherit;
    font-size: 0.8rem;
    outline: none;
  }
`;

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

  const handleKeyDown = async (e) => {
    if (e.key === "Enter") {
      await handleSubmit(e);
    }
  };

  return (
    <StyledCreateComment>
      <input ref={contentRef} placeholder="Write a comment..." onKeyDown={handleKeyDown} />
    </StyledCreateComment>
  );
};

export const Comments = ({ children }) => {
  const { data: comments, isLoading, error } = useAsync(loadChildPosts, { children });

  if (isLoading) return <Spinner />;
  if (error) return <Error message={error} />;

  console.log(comments);

  return (
    <StyledComments>
      {comments.sort((a, b) => moment(b.created).unix() - moment(a.created).unix()).map((comment) => (
        <div key={comment.id} className="comment">
          <div className="main">
            <div className="profile">
              <img
                alt="profile"
                src={`https://eu.ui-avatars.com/api/?rounded=true&bold=true&background=0061ff&color=ffffff&uppercase=true&format=svg&name=${comment.author.id}`}
              />
            </div>
            <div className="content">
              {comment.content.map((block, i) => (
                <StyledBlock key={i}>{renderContent(block)}</StyledBlock>
              ))}
            </div>
          </div>
          <div className="footer">
            <p title={moment(comment.created).format("HH:mma - Do MMMM, YYYY")}>{moment(comment.created).fromNow()}</p>
          </div>
        </div>
      ))}
    </StyledComments>
  );
};
