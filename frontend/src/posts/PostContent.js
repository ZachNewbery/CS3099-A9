import React from "react";
import styled from "styled-components";

import { colors } from "../helpers";
import { MarkdownEditor } from "../components/MarkdownEditor";

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
  margin-top: 1rem;
  max-height: 20rem;
  overflow: hidden;
  position: relative;

  &::after {
    content: "";
    position: absolute;
    width: 100%;
    height: 15px;
    background: linear-gradient(0, white 16%, rgba(255, 255, 255, 0.7) 70%, transparent);
    bottom: 0;
  }

  .header + div {
    padding: 0.5rem;
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

export const renderContent = (content = {}) => {
  return (
    <MarkdownEditor readOnly={true} name="content" defaultValue={content.text?.text || content.markdown?.text} />
  );
};