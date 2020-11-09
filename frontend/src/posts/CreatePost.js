import React, { useState, useRef } from "react";
import { useHistory } from "react-router";
import styled from "styled-components";
import moment from "moment";
import { fetchData } from "../helpers";

const createPost = async ({ title, body }) => {
  const post = {
    title,
    body: [
      {
        content: body,
        contentType: "text"
      }
    ],
    commentsCount: 0,
    likesCount: 0,
    timestamp: moment().toISOString(),
    user: "1"
  }
  
  return fetchData(`${process.env.REACT_APP_API_URL}/posts`, JSON.stringify(post), "POST");
}

const StyledContainer = styled.div`
  width: 500px;
  margin: auto;
`;

export const CreatePost = () => {
  const formRef = useRef(null);
  const history = useHistory();
  const [errors, setErrors] = useState({});

  const handleSubmit = async e => {
    e.preventDefault();
    let currentErrors = {};

    let { title, body } = formRef.current;
    
    title = title.value;
    body = body.value;

    if (title.length < 5) {
      currentErrors.title = "Title is too short"
    }

    if (title.length === 0) {
      currentErrors.title = "Missing title"
    }

    if (body.length < 5) {
      currentErrors.body = "Body is too short"
    }

    if (body.length === 0) {
      currentErrors.body = "Missing body"
    }

    if (Object.keys(currentErrors).length === 0) {
      try {
        await createPost({ title, body })
        return history.push('/')
      } catch (error) {
        currentErrors.body = error.message; // TODO: see how they're passing errors
      }
    }

    setErrors(currentErrors);
  }

  return (
    <StyledContainer>
      <h1>Create Post</h1>
      <form ref={formRef}>
        <label>
          Title:
          <input type='text' name='title' />
          <p>{errors.title}</p>
        </label>
        <label>
          Content:
          <textarea type='text' name='body' />
          <p>{errors.body}</p>
        </label>
        <button onClick={handleSubmit}>Create</button>
      </form>
  </StyledContainer>
  )
}