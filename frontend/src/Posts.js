import React from "react";
import styled from "styled-components";
import moment from "moment";
import { useParams, useHistory } from "react-router-dom";

const CONTENT_TYPES = {
  TEXT: "text",
  IMAGE: "image",
  POLL: "poll"
}

const POSTS_DATA = [
  { 
    id: "0", 
    title: "First Post",
    body: [
      { content: "First Post content", contentType: CONTENT_TYPES.TEXT },
      { content: { url: "https://www.snopes.com/tachyon/2019/11/trump-rocky.png", caption: "Trump as Rocky" }, contentType: CONTENT_TYPES.IMAGE }
    ],
    likesCount: 4,
    comments: [
      { 
        id: "1",
        timestamp: "2020-10-27T12:24:39+00:00",
        body: [
          { content: "This is a comment", contentType: CONTENT_TYPES.TEXT },
          { content: { url: "https://www.snopes.com/tachyon/2019/11/trump-rocky.png", caption: "Trump as Rocky" }, contentType: CONTENT_TYPES.IMAGE }
        ],
        likesCount: 2,
        user: "1",
        edited: true,
      },
      { 
        id: "2",
        timestamp: "2020-10-27T12:24:39+00:00",
        body: [
          { content: "This is another comment", contentType: CONTENT_TYPES.TEXT },
        ],
        likesCount: 3,
        user: "1"
      },
    ],
    timestamp: "2020-10-27T12:24:39+00:00",
    user: "0"
  },
  { 
    id: "1", 
    title: "Second post",
    body: [
      { content: "Second post content", contentType: CONTENT_TYPES.TEXT },
    ],
    likesCount: 7,
    comments: [
      { 
        id: "3",
        timestamp: "2020-10-27T12:24:39+00:00",
        body: [
          { content: "This is a comment", contentType: CONTENT_TYPES.TEXT },
          { content: { url: "https://www.snopes.com/tachyon/2019/11/trump-rocky.png", caption: "Trump as Rocky" }, contentType: CONTENT_TYPES.IMAGE }
        ],
        likesCount: 3,
        user: "1"
      },
      { 
        id: "4",
        timestamp: "2020-10-27T12:24:39+00:00",
        body: [
          { content: "This is another comment", contentType: CONTENT_TYPES.TEXT },
        ],
        likesCount: 2,
        user: "1",
        edited: true
      },
    ],
    timestamp: "2020-10-27T12:24:39+00:00",
    user: "1",
  }
]

const StyledContainer = styled.div`
  margin: auto;
  width: 500px;
  padding: 1.5em 0;
`;

const StyledPost = styled.div`
  cursor: pointer;
  padding: 10px;
  background: white;
  border-radius: 5px;
  border: 1px solid lightgray;
  margin: 0 0 1.5em;
  width: 100%;
  box-sizing: border-box;
  .header {
    display: flex;
    .title {
      font-size: 1.5em;
      flex: 1;
      margin: 0;
    }
  }
  .date-time {
    display: flex;
    flex-flow: column nowrap;
    justify-content: center;
    align-items: flex-end;
    margin-left: 1rem;
  }
  .time {
    color: #676767;
    font-weight: bold;
    margin: 0;
  }
  .date {
    color: #676767;
    margin: 0;
  }
  hr {
    border: none;
    background: rgba(0, 0, 0, 0.1);
    width: 100%;
    height: 1px;
  }
  .stats {
    display: flex;
    justify-content: flex-start;
    align-items: center;
    color: #676767;
    & > * {
      margin: 0;
      margin-right: 0.5em;
      &:hover {
        text-decoration: underline;
      }
    }
  }
`;

const StyledBlock = styled.div`
  padding: 0.8em 0;
  * {
    margin: 0.3em 0 0;
  }
  .image-block {
    img {
    width: 100%;
    height: auto;
    }
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

const StyledComments = styled.div`
  margin: 0;
  font-size: 0.8em;
  & > div {
    padding: 5px 10px;
    cursor: auto;
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

const TextContent = ({ content }) => {
  return <p>{content}</p>
}

const ImageContent = ({ content: { url, caption }  }) => {
  return (
    <div className="image-block">
      <img src={url} alt="Lovely Scenery" />
      <p>{caption}</p>
    </div>
  )
}

const PollContent = ({ content }) => {
  return <div>{content}</div>
}

const renderContent = ({ contentType, content }) => {
  switch (contentType) {
    case CONTENT_TYPES.TEXT:
      return <TextContent content={content} />
    case CONTENT_TYPES.IMAGE:
      return <ImageContent content={content} />
    case CONTENT_TYPES.POLL:
      return <PollContent content={content} />
    default:
      return null;
  }
}

const Post = ({ id, title, body, user, timestamp, comments, likesCount }) => {
  const history = useHistory();
  return (
    <StyledPost onClick={() => history.push(`/post/${id}`)}>
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
        <p>{`${comments.length} comments`}</p>
        <p>{`${likesCount} likes`}</p>
      </div>
    </StyledPost>
  )
}

const Comment = ({ body, timestamp, likesCount, edited }) => {
  return (
    <StyledPost>
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
    </StyledPost>
  )
}

const Comments = ({ comments }) => {
  return (
    <StyledComments>
      {comments.map(comment => <Comment key={comment.id} {...comment} />)}
    </StyledComments>
  )
}

export const SinglePosts = () => {
  const { postId } = useParams();
  const history = useHistory();

  const post = POSTS_DATA.find(post => post.id === postId);

  return (
    <StyledPostContainer>
      <button className="back" onClick={() => history.goBack()}>Back</button>
      <Post {...post} />
      <Comments comments={post.comments} />
    </StyledPostContainer>
  )
}

export const Posts = () => {
  return (
    <StyledContainer>
      {POSTS_DATA.map(post => <Post key={post.id} {...post} />)}
    </StyledContainer>
  )
}