import React, { useState, useRef } from "react";
import styled from "styled-components";

import { Modal } from "../components/Modal";
import { StyledForm, fetchData } from "../helpers";

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
  
  const titleRef = useRef(null);
  const descriptionRef = useRef(null);

  const handleCreate = async () => {
    setLoading(true);
    
    const title = titleRef.current.value;
    const description = descriptionRef.current.value;

    await createCommunity({ title, description });

    refresh(title);
    setLoading(false);
    hide();
  }
  
  if (!show) return null;

  return (
    <Modal title="Create Community" showModal={show} hide={hide}>
      <StyledContainer>
        <StyledForm>
          <label>
            What shall we call it?
            <input ref={titleRef} placeholder="Title" />
          </label>
          <label>
            Describe to others what it'll be about...
            <textarea ref={descriptionRef} type="text" placeholder="Description" />
          </label>
          <button type="button" onClick={handleCreate}>{loading ? "Loading..." : "Create"}</button>
        </StyledForm>
      </StyledContainer>
    </Modal>
  );
};
