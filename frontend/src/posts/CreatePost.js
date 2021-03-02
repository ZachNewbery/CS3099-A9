import React, { useState, useRef } from "react";
import styled from "styled-components";
import moment from "moment";

import { useHistory } from "react-router";
import { fetchData, getCurrentUser, colors, fonts } from "../helpers";
import { StyledForm } from "../helpers/styles";

const createPost = async ({ title, community, content }) => {
  const post = {
    content: [content],
    community: {
      id: community,
    },
    title: title,
    parent: null,
  };

  return fetchData(`${process.env.REACT_APP_API}/posts/create`, JSON.stringify(post), "POST");
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

export const CreatePost = ({ community, host, refresh }) => {
  const formRef = useRef(null);
  const history = useHistory();
  const [errors, setErrors] = useState({});

  const handleSubmit = async (e) => {
    e.preventDefault();
    let currentErrors = {};

    let { title, body } = formRef.current;

    title = title.value;
    body = body.value;

    // if (title.length < 5) {
    //   currentErrors.title = "Title is too short";
    // }

    // if (title.length === 0) {
    //   currentErrors.title = "Missing title";
    // }

    // if (body.length < 5) {
    //   currentErrors.body = "Body is too short";
    // }

    // if (body.length === 0) {
    //   currentErrors.body = "Missing body";
    // }

    if (Object.keys(currentErrors).length === 0) {
      try {
        await createPost({ title, community, content: { text: body } });
        return refresh();
      } catch (error) {
        currentErrors.body = error.message;
      }
    }

    setErrors(currentErrors);
  };

  console.log(errors);

  return (
    <StyledContainer>
      <StyledForm ref={formRef}>
        <label>
          <input type="text" name="title" placeholder="Title" />
        </label>
        <label>
          <textarea type="text" name="body" placeholder="Start writing..." />
        </label>
        <button onClick={handleSubmit}>Post to {community} </button>
      </StyledForm>
    </StyledContainer>
  );
};
