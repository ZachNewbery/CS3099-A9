import React, { useState, useRef } from "react";
import { StyledForm, fetchData } from "../helpers";

const editProfile = async ({ password }) => {
  return await fetchData(`${process.env.REACT_APP_API}/edit_profile`, JSON.stringify({ password }), "PUT");
};

export const ProfileSettings = () => {
  const [loading, setLoading] = useState(false);
  const passwordRef = useRef(null);

  const handleConfirm = async () => {
    const password = passwordRef.current.value;
    setLoading(true);

    const token = await editProfile({ password });

    localStorage.setItem("access-token", token);

    setLoading(false);
  };

  return (
    <StyledForm>
      <label>
        Change Password
        <input type="password" ref={passwordRef} />
      </label>
      <button type="button" onClick={handleConfirm}>
        {loading ? "Loading..." : "Confirm"}
      </button>
    </StyledForm>
  );
};
