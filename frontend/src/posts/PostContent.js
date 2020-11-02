import React from "react";
import styled from "styled-components";

export const StyledBlock = styled.div`
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

export const StyledContent = styled.div`
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

export const CONTENT_TYPES = {
  TEXT: "text",
  IMAGE: "image",
  POLL: "poll"
}

export const renderContent = ({ contentType, content }) => {
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