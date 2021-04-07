import React, { useState, useRef } from "react";

import { Modal } from "../components/Modal";
import { StyledForm, fetchData, getFormValues } from "../helpers";
import { MarkdownEditor } from "../components/MarkdownEditor";
import { Tooltip } from "../components/Tooltip";

const editPost = async ({ id, title, content }) => {
  return fetchData(`${process.env.REACT_APP_API}/posts/${id}`, JSON.stringify({ title, content }), "PATCH");
};

export const EditPost = ({ show, hide, id, initialTitle, initialContent, refresh }) => {
  const [loading, setLoading] = useState(false);
  const [errors, setErrors] = useState({});

  const formRef = useRef(null);

  if (!show) return null;

  const handleSubmit = async (e) => {
    e.preventDefault();
    let currentErrors = {};

    setLoading(true);

    let { title, ...content } = getFormValues(formRef.current);

    if (title.length < 5) {
      currentErrors.title = "Title is too short";
    }

    if (title.length === 0) {
      currentErrors.title = "Missing title";
    }

    content = Object.entries(content);

    for (const [key, value] of content) {
      if (value.length < 5) {
        currentErrors[key] = "Content is too short";
      }

      if (value.length === 0) {
        currentErrors[key] = "Missing content";
      }
    }

    content = content.map(([key, value]) => ({ [key.split("-")[1]]: value }));

    if (Object.keys(currentErrors).length === 0) {
      try {
        await editPost({ title, content, id });

        setLoading(false);
        refresh();
        return hide();
      } catch (error) {
        currentErrors.title = error.message;
      }
    }

    setLoading(false);
    setErrors(currentErrors);
  };

  return (
    <Modal title="Edit Post" showModal={show} hide={hide} childrenStyle={{ padding: "2rem" }} enterKeySubmits={false}>
      <StyledForm style={{ width: "100%" }} ref={formRef} onChange={() => setErrors({})}>
        <label>
          Title
          <input name="title" defaultValue={initialTitle} />
          {errors.title && <Tooltip text={errors.title}  style={{ top: "1.75rem" }} />}
        </label>
        {initialContent.map((content, i) => {
          const contentType = Object.keys(content)[0];
          const key = `content-${contentType}-${i}`;
          return (
            <label key={i}>
              <MarkdownEditor name={key} defaultValue={content[contentType]} />
              {errors[key] && <Tooltip text={errors[key]} style={{ top: "3rem" }} />}
            </label>
          );
        })}
        <button onClick={handleSubmit}>{loading ? "Loading..." : "Change"}</button>
      </StyledForm>
    </Modal>
  );
};
