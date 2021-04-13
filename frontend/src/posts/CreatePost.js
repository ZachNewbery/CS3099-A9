import React, { useState, useContext, useRef } from "react";
import styled from "styled-components";

import { InstanceContext, CommunityContext } from "../App";

import { fetchData, colors, fonts } from "../helpers";
import { StyledForm } from "../helpers/styles";
import { Tooltip } from "../components/Tooltip";
import { MarkdownEditor } from "../components/MarkdownEditor";

const createPost = async ({ title, community, instance, content }) => {
  const post = {
    content: [content],
    community: {
      id: community,
    },
    title: title,
    parent: null,
  };

  const url = new URL(`${process.env.REACT_APP_API}/posts/create`);
  const appendParam = (key, value) => value && url.searchParams.append(key, value);
  appendParam("host", instance);
  return fetchData(url, JSON.stringify(post), "POST");
};

const StyledContainer = styled.div`
  display: flex;
  flex-flow: column nowrap;
  width: 100%;
  background: white;
  border: 1px solid ${colors.mediumLightGray};
  border-radius: 0.6rem;
  padding: 1rem;
  & > h1 {
    margin: 0 0 0.5rem;
    font-family: ${fonts.accent};
    font-weight: normal;
    font-size: 1.25rem;
    letter-spacing: 0.5px;
    border-bottom: 1px solid ${colors.veryLightGray};
  }
  & > form {
    width: 100%;
    & > label {
      margin: 0;
      & > input {
        margin: 0 0 0.25rem;
      }
    }
    & > button {
      margin-top: 0.5rem;
    }
  }
`;

export const CreatePost = ({ refresh }) => {
  const { instance } = useContext(InstanceContext);
  const { community } = useContext(CommunityContext);
  
  const formRef = useRef(null);
  const [title, setTitle] = useState("");
  const [body, setBody] = useState("");
  const [errors, setErrors] = useState({});

  const handleSubmit = async (e) => {
    e.preventDefault();
    let currentErrors = {};

    if (title.length < 5) {
      currentErrors.title = "Title is too short";
    }

    if (title.length === 0) {
      currentErrors.title = "Missing title";
    }

    if (body.length < 5) {
      currentErrors.body = "Body is too short";
    }

    if (body.length === 0) {
      currentErrors.body = "Missing body";
    }

    if (Object.keys(currentErrors).length === 0) {
      try {
        await createPost({ title, community, instance, content: { markdown: { text: body } } });
        setTitle("");
        setBody("");
        setErrors({});
        return refresh();
      } catch (error) {
        currentErrors.body = error.message;
      }
    }

    setErrors(currentErrors);
  };

  return (
    <StyledContainer>
      <StyledForm ref={formRef} onChange={() => setErrors({})}>
        <label>
          <input type="text" name="title" placeholder="Title" value={title} onChange={(e) => setTitle(e.target.value)} />
          {errors.title && <Tooltip text={errors.title} />}
        </label>
        {title && (
          <label>
            <MarkdownEditor name="body" defaultValue={body} onChange={(e) => setBody(e)}/>
            {errors.body && <Tooltip text={errors.body} />}
          </label>
        )}
        <button onClick={handleSubmit} title={`Post to ${community}`}><p>Post to {community}</p></button>
      </StyledForm>
    </StyledContainer>
  );
};
