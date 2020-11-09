import React from "react";
import styled from "styled-components";
import moment from "moment";
import { useParams, useHistory } from "react-router-dom";
import { useAsync } from "react-async";
import { fetchData } from "../helpers";
import { StyledBlock, StyledContent, renderContent } from "./PostContent";
import { Comments, CreateComment } from "./Comments";

const loadSinglePost = async ({ id }) => {
  return fetchData(`${process.env.REACT_APP_API_URL}/posts/${id}`);
}

const StyledPostContainer = styled.div`
  width: 500px;
  margin: auto;
  .back {
    cursor: pointer;
    margin: 0.8em 0;
    background: white;
    border-radius: 0.3em;
    border: 1px solid lightgray;
    transition: all 0.3s;
    &:hover {
      background: lightgray;
    }
  }
`;

export const Post = ({ id, title, body, user, timestamp, commentsCount, likesCount }) => {
  const history = useHistory();
  return (
    <StyledContent onClick={() => history.push(`/post/${id}`)}>
      <div className="header">
        <h1 className="title">{title}</h1>
        <div className="date-time">
          <p className="time">{moment(timestamp).format("HH:mm")}</p>
          <p className="date">{moment(timestamp).format("DD MMMM YYYY")}</p>
        </div>
      </div>
      {body.map((block, i) => (
        <StyledBlock key={i}>
          {renderContent(block)}
        </StyledBlock>
      ))}
      <hr />
      <div className="stats">
        <p>{`${commentsCount} comments`}</p>
        <p>{`${likesCount} likes`}</p>
      </div>
    </StyledContent>
  )
}

export const SinglePost = () => {
  const { postId } = useParams();
  const history = useHistory();

  const { data, isLoading, reload } = useAsync(loadSinglePost, { id: postId });

  if (isLoading) {
    return <h1>Loading</h1>
  }

  return (
    <StyledPostContainer>
      <button className="back" onClick={() => history.goBack()}>Back</button>
      <Post {...data} />
      <CreateComment postId={data.id} refresh={reload} />
      <Comments postId={data.id} />
    </StyledPostContainer>
  )
}