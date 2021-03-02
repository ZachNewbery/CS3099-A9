import React, { useState, useRef } from "react";
import styled from "styled-components";

import { Modal } from "../components/Modal";
import { StyledForm, fetchData } from "../helpers";

const editCommunity = async ({ title, description, id }) => {
  return await fetchData(`${process.env.REACT_APP_API}/communities/${id}`, JSON.stringify({ title, description }), "PATCH");
};

export const EditCommunity = ({ show, hide, id, initialTitle, initialDescription, refresh }) => {
  const [loading, setLoading] = useState(false);

  const titleRef = useRef(null);
  const descriptionRef = useRef(null);

  if (!show) return null;

  const handleSubmit = async e => {
    e.preventDefault();

    setLoading(true);

    const title = titleRef.current.value;
    const description = descriptionRef.current.value;

    await editCommunity({ title, description, id });

    setLoading(false);
    refresh();
    hide();
  };

  return (
    <Modal title="Edit Community" showModal={show} hide={hide} childrenStyle={{ padding: "2rem" }}>
      <StyledForm style={{ width: "100%" }}>
        <label>
          Title
          <input ref={titleRef} defaultValue={initialTitle} />
        </label>
        <label>
          Description
          <input ref={descriptionRef} defaultValue={initialDescription} />
        </label>
        <button onClick={handleSubmit}>{loading ? "Loading..." : "Change"}</button>
      </StyledForm>
    </Modal>
  );
};
