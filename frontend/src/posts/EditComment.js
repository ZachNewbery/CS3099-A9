import React, { useState, useRef } from "react";

import { Modal } from "../components/Modal";
import { StyledForm, fetchData, getFormValues } from "../helpers";
import { MarkdownEditor } from "../components/MarkdownEditor";

const editComment = async ({ id, content }) => {
  return fetchData(`${process.env.REACT_APP_API}/posts/${id}`, JSON.stringify({ content }), "PATCH");
};

export const EditComment = ({ show, hide, id, initialTitle, initialContent, refresh }) => {
  const [loading, setLoading] = useState(false);

  const formRef = useRef(null);

  if (!show) return null;

  const handleSubmit = async (e) => {
    e.preventDefault();

    setLoading(true);

    let content = getFormValues(formRef.current);

    content = Object.entries(content).map(([key, value]) => ({ [key.split("-")[1]]: { text: value } }));
    
    await editComment({ content, id });

    setLoading(false);
    refresh();
    hide();
  };

  return (
    <Modal title="Edit Comment" showModal={show} hide={hide} childrenStyle={{ padding: "2rem" }}>
      <StyledForm style={{ width: "100%" }} ref={formRef}>
        {initialContent.map((content, i) => {
          const contentType = Object.keys(content)[0];
          return (
            <label key={i}>
              <MarkdownEditor name={`content-${contentType}-${i}`} defaultValue={content[contentType].text} />
            </label>
          );
        })}
        <button onClick={handleSubmit}>{loading ? "Loading..." : "Change"}</button>
      </StyledForm>
    </Modal>
  );
};
