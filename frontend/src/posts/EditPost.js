import React, { useState, useRef } from "react";

import { Modal } from "../components/Modal";
import { StyledForm, fetchData } from "../helpers";

const editPost = async ({ id, title, content }) => {
  return fetchData(`${process.env.REACT_APP_API}/posts/${id}`, JSON.stringify({ title, content }), "PATCH");
}

export const EditPost = ({ show, hide, id, initialTitle, initialContent, refresh }) => {
  const [loading, setLoading] = useState(false);

  const titleRef = useRef(null);
  const contentRef = useRef(null);

  if (!show) return null;

  const handleSubmit = async e => {
    e.preventDefault();

    setLoading(true);

    const title = titleRef.current.value;
    const text = contentRef.current.value;
    const content = [
      { type: "text", value: text }
    ]

    await editPost({ title, content, id });

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
          Content
          <input ref={contentRef} defaultValue={initialContent} />
        </label>
        <button onClick={handleSubmit}>{loading ? "Loading..." : "Change"}</button>
      </StyledForm>
    </Modal>
  );
};
