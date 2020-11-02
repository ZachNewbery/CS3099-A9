import React, { useState, useRef } from "react";
import { useHistory } from "react-router";
import styled from "styled-components";

const StyledContainer = styled.div`
  width: 500px;
  margin: auto;
`;

export const CreatePost = () => {
  const formRef = useRef(null);
  const history = useHistory();
  const [errors, setErrors] = useState({});

  const handleSubmit = e => {
    e.preventDefault();
    let currentErrors = {};
    setErrors(currentErrors);

    let { title, body } = formRef.current;
    
    title = title.value;
    body = body.value;

    history.push('/');

    setErrors(currentErrors);
    console.log({ title, body });
  }

  return (
    <StyledContainer>
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