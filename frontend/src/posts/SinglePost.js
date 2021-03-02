import React from "react";
import styled from "styled-components";
import moment from "moment";
import { useParams, useHistory } from "react-router-dom";
import { useAsync } from "react-async";
import { fetchData, Spinner, colors, fonts } from "../helpers";
import { StyledBlock, StyledContent, renderContent } from "./PostContent";
import { Comments, CreateComment } from "./Comments";

const loadSinglePost = async ({ id }) => {
  return fetchData(`${process.env.REACT_APP_API}/posts/${id}`);
};

const StyledPostContainer = styled.div`
  width: 100%;
  .back {
    cursor: pointer;
    margin: 0.8em 0;
    background: ${colors.blueGradient};
    box-shadow: ${colors.blueInsetShadow};
    font-family: ${fonts.accent};
    letter-spacing: 0.4px;
    color: ${colors.white};
    padding: 0.3rem 0.5rem;
    font-size: 0.8rem;
    border-radius: 0.3em;
    border: 1px solid lightgray;
    transition: all 0.3s;
    &:hover {
      background: lightgray;
    }
  }
`;

export const Post = ({ id, title, content, user, created, children }) => {
  const history = useHistory();
  return (
    <StyledContent onClick={() => history.push(`/post/${id}`)}>
      <div className="header">
        <h1 className="title">{title}</h1>
        <div className="date-time">
          <p className="time">{moment(created).format("HH:mm")}</p>
          <p className="date">{moment(created).format("DD MMMM YYYY")}</p>
        </div>
      </div>
      {content.map((block, i) => (
        <StyledBlock key={i}>{renderContent(block)}</StyledBlock>
      ))}
      <hr />
      <div className="stats">
        <p>{`${children.length} comments`}</p>
      </div>
    </StyledContent>
  );
};

export const SinglePost = () => {
  const { postId } = useParams();
  const history = useHistory();

  const { data, isLoading, reload } = useAsync(loadSinglePost, { id: postId });

  if (isLoading) {
    return <Spinner />;
  }

  return (
    <StyledPostContainer>
      <button className="back" onClick={() => history.goBack()}>
        Back
      </button>
      <Post {...data} />
      <CreateComment postId={data.id} refresh={reload} communityId={data.community.id} />
      <Comments children={data.children} />
    </StyledPostContainer>
  );
};
