import React from "react";
import styled from "styled-components";

import { colors } from "../helpers";

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
  display: flex;
  height: 100%;
  flex-flow: column nowrap;
  background: white;
  border: 1px solid ${colors.mediumLightGray};
  border-radius: 0.6rem;
  padding: 1rem;
  margin-top: 1rem;

  .header {
    display: flex;
    .title {
      font-size: 1.5em;
      flex: 1;
      margin: 0;
      width: 10rem;
      overflow: hidden;
      white-space: nowrap;
      text-overflow: ellipsis;
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

export const renderContent = (content = []) => {
  console.log(content);
  return (
    <>
      {content.text && <TextContent content={content.text} />}
      {content.markdown && <TextContent content={content.markdown} />}
    </>
  );
};

const TextContent = ({ content }) => {
  return <p>{content}</p>;
};
