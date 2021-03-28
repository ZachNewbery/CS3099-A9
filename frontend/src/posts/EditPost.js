import React, { useState, useRef } from "react";

import { Modal } from "../components/Modal";
import { StyledForm, fetchData } from "../helpers";
import { MarkdownEditor } from "../components/MarkdownEditor";
import { Tooltip } from "../components/Tooltip";

const editPost = async ({ id, title, content }) => {
  return fetchData(`${process.env.REACT_APP_API}/posts/${id}`, JSON.stringify({ title, content }), "PATCH");
};

export const EditPost = ({ show, hide, id, initialTitle, initialContent, refresh }) => {
  const [loading, setLoading] = useState(false);
  const [errors, setErrors] = useState({});

  const titleRef = useRef(null);
  const contentRef = useRef(null);

  if (!show) return null;

  const handleSubmit = async (e) => {
    e.preventDefault();
    let currentErrors = {};

    setLoading(true);

    const title = titleRef.current.value;
    const text = contentRef.current.props.value;

    if (title.length < 5) {
      currentErrors.title = "Title is too short";
    }

    if (title.length === 0) {
      currentErrors.title = "Missing title";
    }

    if (text.length < 5) {
      currentErrors.text = "Body is too short";
    }

    if (text.length === 0) {
      currentErrors.text = "Missing body";
    }

    const content = [
      { markdown: text }, // TODO
    ];

    if (Object.keys(currentErrors).length === 0) {
      try {
        await editPost({ title, content, id });

        setLoading(false);
        refresh();
        return hide();
      } catch (error) {
        currentErrors.text = error.message;
      }
    }

    setErrors(currentErrors);
  };

  return (
    <Modal title="Edit Post" showModal={show} hide={hide} childrenStyle={{ padding: "2rem" }} enterKeySubmits={false}>
      <StyledForm style={{ width: "100%" }}>
        <label>
          Title
          <input ref={titleRef} defaultValue={initialTitle} />
          {errors.title && <Tooltip text={errors.title} />}
        </label>
        <label>
          <MarkdownEditor ref={contentRef} defaultValue={initialContent} />
          {errors.text && <Tooltip text={errors.text} />}
        </label>
        <button onClick={handleSubmit}>{loading ? "Loading..." : "Change"}</button>
      </StyledForm>
    </Modal>
  );
};
