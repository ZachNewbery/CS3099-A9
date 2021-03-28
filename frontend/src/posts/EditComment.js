import React, { useState, useRef } from "react";

import { Modal } from "../components/Modal";
import { StyledForm, fetchData } from "../helpers";

const editComment = async ({ id, content }) => {
  return fetchData(`${process.env.REACT_APP_API}/posts/${id}`, JSON.stringify({ content }), "PATCH");
};

export const EditComment = ({ show, hide, id, initialTitle, initialContent, refresh }) => {
  const [loading, setLoading] = useState(false);

  const contentRef = useRef(null);

  if (!show) return null;

  const handleSubmit = async (e) => {
    e.preventDefault();

    setLoading(true);

    const text = contentRef.current.value;
    const content = [
      { text: text }, // TODO
    ];

    await editComment({ content, id });

    setLoading(false);
    refresh();
    hide();
  };

  return (
    <Modal title="Edit Comment" showModal={show} hide={hide} childrenStyle={{ padding: "2rem" }}>
      <StyledForm style={{ width: "100%" }}>
        <label>
          <input ref={contentRef} defaultValue={initialContent} />
        </label>
        <button onClick={handleSubmit}>{loading ? "Loading..." : "Change"}</button>
      </StyledForm>
    </Modal>
  );
};
