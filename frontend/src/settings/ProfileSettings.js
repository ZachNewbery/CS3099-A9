import React, { useState, useRef } from "react";
import { StyledForm, fetchData } from "../helpers";

const editProfile = async ({ password }) => {
  return await fetchData(`${process.env.REACT_APP_API}/edit_profile`, JSON.stringify({ password }), "PUT");
};

export const ProfileSettings = () => {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);
  
  const passwordRef = useRef(null);
  const confirmPasswordRef = useRef(null);

  const handleConfirm = async () => {
    const password = passwordRef.current.value;
    const confirmPassword = confirmPasswordRef.current.value;

    let errors = [];
    
    if (password.length < 5) {
      errors.push("Password too short");
    }

    if (password !== confirmPassword) {
      errors.push("Passwords don't match");
    }

    setError(errors.join("; "));

    if (Object.keys(errors).length !== 0) {
      return;
    }
    

    setLoading(true);

    const result = await editProfile({ password });

    localStorage.setItem("access-token", result.token);

    setLoading("done");

    passwordRef.current.value = "";
    confirmPasswordRef.current.value = "";
  };

  return (
    <StyledForm>
      <label>
        Change Password
        <input type="password" ref={passwordRef} />
        <p className="error">{error}</p>
      </label>
      <label>
        Confirm Password
        <input type="password" ref={confirmPasswordRef} />
      </label>
      <button type="button" onClick={handleConfirm}>
        {loading === true ? "Loading..." : loading === "done" ? "done!" : "Confirm"}
      </button>
    </StyledForm>
  );
};
