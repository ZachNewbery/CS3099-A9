import React, { useState, useRef } from "react";

import { Modal } from "../components/Modal";
import { StyledForm, fetchData } from "../helpers";
import { Tooltip } from "../components/Tooltip";

const editCommunity = async ({ title, description, id }) => {
  return await fetchData(`${process.env.REACT_APP_API}/communities/${id}`, JSON.stringify({ title, description }), "PATCH");
};

export const EditCommunity = ({ show, hide, id, initialTitle, initialDescription, refresh }) => {
  const [loading, setLoading] = useState(false);
  const [errors, setErrors] = useState({});

  const titleRef = useRef(null);
  const descriptionRef = useRef(null);

  if (!show) return null;

  const handleSubmit = async e => {
    e.preventDefault();
    let currentErrors = {};

    setLoading(true);

    const title = titleRef.current.value;
    const description = descriptionRef.current.value;

    if (title.length < 5) {
      currentErrors.title = "Too short";
    }

    if (title.length === 0) {
      currentErrors.title = "No title";
    }

    if (description.length < 5) {
      currentErrors.description = "Too short";
    }

    if (description.length === 0) {
      currentErrors.description = "No description";
    }

    if (Object.keys(currentErrors).length === 0) {
      try {
        await editCommunity({ title, description, id });

        setLoading(false);
        refresh(title);
        return hide();
      } catch (error) {
        currentErrors.text = error.message;
      }
    }

    setLoading(false);
    setErrors(currentErrors);
  };

  return (
    <Modal title="Edit Community" showModal={show} hide={hide} childrenStyle={{ padding: "2rem" }}>
      <StyledForm style={{ width: "100%" }} onChange={() => setErrors({})}>
        <label>
          Title
          <input ref={titleRef} defaultValue={initialTitle} />
          {errors.title && <Tooltip text={errors.title} />}
        </label>
        <label>
          Description
          <textarea ref={descriptionRef} defaultValue={initialDescription} />
          {errors.description && <Tooltip text={errors.description} />}
        </label>
        <button onClick={handleSubmit}>{loading ? "Loading..." : "Change"}</button>
      </StyledForm>
    </Modal>
  );
};
