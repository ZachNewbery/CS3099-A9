import React, { useState, useRef } from "react";

import { Modal } from "../components/Modal";
import { StyledForm, fetchData, getFormValues } from "../helpers";
import { MarkdownEditor } from "../components/MarkdownEditor";
import { Tooltip } from "../components/Tooltip";

const editComment = async ({ id, content, instance, community }) => {
  const url = new URL(`${process.env.REACT_APP_API}/posts/${id}`);
  const appendParam = (key, value) => value && url.searchParams.append(key, value);
  appendParam("host", instance);
  appendParam("community", community);
  return fetchData(url, JSON.stringify({ content }), "PATCH");
};

export const EditComment = ({ show, hide, id, initialTitle, initialContent, refresh, instance, community }) => {
  const [loading, setLoading] = useState(false);
  const [errors, setErrors] = useState({});

  const formRef = useRef(null);

  if (!show) return null;

  const handleSubmit = async (e) => {
    e.preventDefault();
    let currentErrors = {};

    setLoading(true);

    let content = getFormValues(formRef.current);
    content = Object.entries(content).map(([key, value]) => ({ [key.split("-")[1]]: { text: value } }));

    let i = 0;
    for (const item of content) {
      const contentType = Object.keys(item)[0];
      const value = (item[contentType] || {}).text;
      if (value.length < 5) {
        currentErrors[`content-${contentType}-${i}`] = "Too short";
      }
      i++;
    }

    if (Object.keys(currentErrors).length === 0) {
      try {
        await editComment({ content, id, instance, community });
        refresh();
        setLoading(false);
        return hide();
      } catch (error) {
        currentErrors.content = error.message;
      }
    }

    setErrors(currentErrors);
    setLoading(false);
  };

  return (
    <Modal title="Edit Comment" showModal={show} hide={hide} childrenStyle={{ padding: "2rem" }}>
      <StyledForm style={{ width: "100%" }} ref={formRef}>
        {initialContent.map((content, i) => {
          const contentType = Object.keys(content)[0];
          const name = `content-${contentType}-${i}`;
          return (
            <label key={i}>
              <MarkdownEditor name={name} defaultValue={content[contentType].text} />
              {errors[name] && <Tooltip text={errors[name]} />}
            </label>
          );
        })}
        <button onClick={handleSubmit}>{loading ? "Loading..." : "Change"}</button>
      </StyledForm>
    </Modal>
  );
};
