import React, { useState, useRef } from "react";
import { useHistory } from "react-router";
import styled from "styled-components";
import moment from "moment";
import { fetchData, getCurrentUser } from "../helpers";

const createPost = async ({ title, communityId, content }) => {
  // author: {id: "Fraser", host: "Hostname"}
  // community: "Community Name"
  // content: [{text: {text: "Some amazing post content!"}}]
  // parentPost: "bb6964f3-a1d3-4007-ad48-a9116b801600"
  // title: "A Title"
  
  const post = {
    content: [
      content
    ],
    community: communityId,
    title: title,
    author: getCurrentUser()
  };

  return fetchData(
    `${process.env.REACT_APP_API}/posts`,
    JSON.stringify(post),
    "POST"
  );
}

const StyledContainer = styled.div`
  width: 500px;
  margin: auto;
`;

export const CreatePost = ({ communityId }) => {
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
        await createPost({ title, communityId, content: { markdown: { text: body } } })
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