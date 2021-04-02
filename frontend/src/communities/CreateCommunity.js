import React, { useState, useRef } from "react";
import styled from "styled-components";

import { Modal } from "../components/Modal";
import { StyledForm, fetchData } from "../helpers";
import { Tooltip } from "../components/Tooltip";

const createCommunity = async ({ title, description }) => {
  return await fetchData(`${process.env.REACT_APP_API}/communities/create`, JSON.stringify({ id: title, title, description }), "POST");
}

const StyledContainer = styled.div`
  width: 100%;
  padding: 1rem 0;
  & > form {
    width: 100%;
    & > label {
      width: 100%;
      margin: 0 0 0.5rem;
    }
    & > button {
      width: 10rem;
      margin-top: 0.5rem;
    }
  }
`;

export const CreateCommunity = ({ show, hide, refresh }) => {
  const [loading, setLoading] = useState(false);
  const [errors, setErrors] = useState({});
  
  const titleRef = useRef(null);
  const descriptionRef = useRef(null);

  const handleCreate = async () => {
    setLoading(true);
    let currentErrors = {};
    
    const title = titleRef.current.value;
    const description = descriptionRef.current.value;

    if (title.length < 5) {
      currentErrors.title = "Too short";
      setLoading(false);
    }

    if (title.length === 0) {
      currentErrors.title = "No title";
      setLoading(false);
    }

    if (description.length < 5) {
      currentErrors.description = "Too short";
      setLoading(false);
    }

    if (description.length === 0) {
      currentErrors.description = "No description";
      setLoading(false);
    }

    if (Object.keys(currentErrors).length === 0) {
      try {
        await createCommunity({ title, description });

        setLoading(false);
        refresh(title);
        return hide();
      } catch (error) {
        currentErrors.text = error.message;
      }
    }

    setErrors(currentErrors);
  }
  
  if (!show) return null;

  return (
    <Modal title="Create Community" showModal={show} hide={hide}>
      <StyledContainer>
        <StyledForm onChange={() => setErrors({})}>
          <label>
            What shall we call it?
            <input ref={titleRef} placeholder="Title" />
            {errors.title && <Tooltip text={errors.title} />}
          </label>
          <label>
            Describe to others what it'll be about...
            <textarea ref={descriptionRef} type="text" placeholder="Description" />
            {errors.description && <Tooltip text={errors.description} />}
          </label>
          <button type="button" onClick={handleCreate}>{loading ? "Loading..." : "Create"}</button>
        </StyledForm>
      </StyledContainer>
    </Modal>
  );
};
