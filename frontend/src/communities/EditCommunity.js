import React, { useState, useRef, useContext } from "react";

import { Modal } from "../components/Modal";
import { StyledForm, fetchData } from "../helpers";
import { Tooltip } from "../components/Tooltip";

import { InstanceContext } from "../App";

const editCommunity = async ({ title, description, id, instance }) => {
  const url = new URL(`${process.env.REACT_APP_API}/communities/${id}`);
  const appendParam = (key, value) => value && url.searchParams.append(key, value);
  appendParam("host", instance);
  return await fetchData(url, JSON.stringify({ title, description, id }), "PATCH");
};

export const EditCommunity = ({ show, hide, id, initialTitle, initialDescription, refresh }) => {
  const [loading, setLoading] = useState(false);
  const [errors, setErrors] = useState({});

  const [title, setTitle] = useState(initialTitle);
  const [description, setDescription] = useState(initialDescription);

  const { instance } = useContext(InstanceContext);
  
  if (!show) return null;

  const handleSubmit = async e => {
    e.preventDefault();
    let currentErrors = {};

    setLoading(true);

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
        await editCommunity({ title, description, id, instance });

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
          <input onChange={e => setTitle(e.target.value)} defaultValue={initialTitle} />
          {errors.title && <Tooltip text={errors.title} />}
        </label>
        <label>
          Description
          <textarea onChange={e => setDescription(e.target.value)} defaultValue={initialDescription} />
          {errors.description && <Tooltip text={errors.description} />}
        </label>
        <button onClick={handleSubmit}>{loading ? "Loading..." : "Change"}</button>
      </StyledForm>
    </Modal>
  );
};
