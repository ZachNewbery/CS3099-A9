import React from "react";
import styled from "styled-components";
import moment from "moment";
import { useParams, useHistory } from "react-router-dom";
import { useAsync } from "react-async";
import { fetchData } from "../helpers";
import { StyledBlock, StyledContent, renderContent } from "./PostContent";

const loadSinglePost = async ({ id }) => {
  return fetchData(`${process.env.REACT_APP_API_URL}/posts/${id}`);
}

const loadComments = async ({ postId }) => {
  return fetchData(`${process.env.REACT_APP_API_URL}/posts/${postId}/comments`);
}

const StyledComments = styled.div`
  margin: 0;
  font-size: 0.8em;
  & > div {
    padding: 5px 10px;
    cursor: auto;
    background-color: #F8F9F9;
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

const Comment = ({ body, timestamp, likesCount, edited }) => {
  return (
    <StyledContent>
      {body.map((block, i) => (
        <StyledBlock key={i} style={{ padding: "0.3em 0", fontSize: "1.1em" }}>
          {renderContent(block)}
        </StyledBlock>
      ))}
      <hr />
      <div className="header">
        <p className="user">Bob</p>
        <div className="date-time">
          <p className="date" style={{marginRight: "0.5em" }}>{`${edited ? "Edited" : ""} ${moment(timestamp).fromNow()}`}</p>
          <p className="time">{`${likesCount} likes`}</p>
        </div>
      </div>
    </StyledContent>
  )
}

const Comments = ({ postId }) => {
  const { data, isLoading } = useAsync(loadComments, { postId });

  if (isLoading) {
    return <h1>Loading</h1>
  }
  
  return (
    <StyledComments>
      {data.map(comment => <Comment key={comment.id} {...comment} />)}
    </StyledComments>
  )
}

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

  const { data, isLoading } = useAsync(loadSinglePost, { id: postId });

  if (isLoading) {
    return <h1>Loading</h1>
  }

  return (
    <StyledPostContainer>
      <button className="back" onClick={() => history.goBack()}>Back</button>
      <Post {...data} />
      <Comments postId={data.id} />
    </StyledPostContainer>
  )
}