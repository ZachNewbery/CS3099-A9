import React, { useState, useEffect } from "react";
import styled from "styled-components";
import ReactMde from "react-mde";
import { Converter } from "showdown";
import { colors, fonts } from "../helpers";

import "../helpers/editor.css";

const converter = new Converter({
  tables: true,
  simplifiedAutoLink: true,
  strikethrough: true,
  tasklists: true,
  emoji: true,
});

const StyledEditor = styled.div`
  padding: 0;
  overflow: hidden;

  && {
    .react-mde {
      border: none;
      width: 100%;
      margin: 0;

      .invisible {
        display: none;
      }

      .mde-text {
        outline: none;
        color: ${colors.text};
        font: 1rem ${fonts.default};
        &::-webkit-scrollbar {
          width: 6px;
          height: 6px;
        }

        &::-webkit-scrollbar-thumb {
          border-radius: 3px;
          background: rgba(200, 200, 200, 0.3);
          transition: background 0.3s;
          &:hover {
            background: rgba(200, 200, 200, 0.6);
          }
        }
      }
      .mde-tabs {
        button {
          cursor: pointer;
          border: none !important;
          outline: none;
          border-radius: 0.25rem;
          font: inherit;
          font-size: 0.8rem;
          font-weight: bold;
          transition: background 0.3s;
          &:not(.selected) {
            font-weight: normal;
          }
        }
      }
      .mde-header-item {
        button {
          color: ${colors.lightText} !important;
          outline: none;
          font: icon;
        }
        .react-mde-dropdown {
          background-color: red !important;
          border-color: red !important;
          border-radius: 0.25rem;
          &::before {
            border-bottom-color: none;
          }
          &::after {
            border-bottom-color: red !important;
          }
          button {
            font-family: ${fonts.default};
            p:hover {
              color: ${colors.text} !important;
            }
          }
        }
      }
      .grip,
      .mde-header {
        display: ${(props) => props.hideheader && "none"};
        height: unset;
      }
      .mde-header-group {
        padding-right: 0;
      }
      .mde-preview {
        margin: 0;
      }
      .mde-preview-content {
        padding: 0.5rem;
        margin: 0;
        white-space: pre-wrap;
        * {
          white-space: pre-wrap;
          font-size: 1rem;
          border-bottom-width: 2px;
          line-height: 1.2;
        }
        hr {
          border: none;
          height: 1px;
          width: 100%;
          background: ${colors.lightText};
          opacity: 0.7;
        }
        p {
          color: ${colors.lightText};
        }
        code {
          background: rgba(0, 0, 0, 0.1);
          margin: 2px;
        }
        blockquote {
          border-left-color: ${colors.lightText};
          * {
            color: ${colors.lightText};
          }
          opacity: 0.5;
          blockquote {
            opacity: 1 !important;
          }
        }
        pre {
          background: rgba(0, 0, 0, 0.1);
          border: 1px solid rgba(0, 0, 0, 0.1);
        }
        ol {
          color: ${colors.lightText};
        }
        h2 {
          color: ${colors.lightText};
        }
        h1 {
          font-size: 1.25rem;
        }
      }
    }
  }
`;

export const MarkdownEditor = React.forwardRef(({ name, tab = "write", style, readOnly, defaultValue, hideButtons = false, subtle }, ref) => {
  const [value, setValue] = useState(defaultValue);
  const [selectedTab, setSelectedTab] = useState(readOnly ? "preview" : tab);

  useEffect(() => {
    setValue(defaultValue);
  }, [defaultValue]);

  useEffect(() => {
    if (!readOnly) setSelectedTab("write");
  }, [readOnly]);

  return (
    <StyledEditor style={{ ...style, boxShadow: subtle && "none" }} className="editor-container" hideheader={readOnly}>
      <ReactMde
        ref={ref}
        value={value}
        onChange={setValue}
        selectedTab={!readOnly && selectedTab}
        onTabChange={setSelectedTab}
        generateMarkdownPreview={(markdown) => Promise.resolve(converter.makeHtml(markdown))}
        childProps={{
          textArea: { name },
          writeButton: { style: { display: hideButtons && "none" } },
          previewButton: { style: { display: hideButtons && "none" } },
        }}
      />
    </StyledEditor>
  );
});
