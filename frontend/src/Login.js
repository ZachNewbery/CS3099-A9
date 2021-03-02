import React, { useRef, useState } from "react";
import styled from "styled-components";
import { Link, Redirect, useHistory } from "react-router-dom";
import { isAuthenticated, fetchData, colors, fonts, StyledForm } from "./helpers";
import { Logo } from "./assets/Logo";

const getUserToken = async ({ email, password }) => {
  return await fetchData(`${process.env.REACT_APP_API}/login`, JSON.stringify({ email, password }), "POST");
};

export const Login = () => {
  const emailRef = useRef(null);
  const passwordRef = useRef(null);
  const [error, setError] = useState();
  const history = useHistory();

  const handleSubmit = async () => {
    const email = emailRef.current.value;
    const password = passwordRef.current.value;

    try {
      const { token } = await getUserToken({ email, password });
      localStorage.setItem("access-token", token);
      return history.push("/");
    } catch (error) {
      setError("Please check email or password is entered correctly.");
    }
  };

  if (isAuthenticated()) return <Redirect to="/" />;

  return (
    <StyledForm onChange={() => setError(null)}>
      <Logo />
      <label>
        Email
        <input type="email" ref={emailRef} name="email" />
      </label>
      <label>
        Password
        <input type="password" ref={passwordRef} name="password" />
        {error && <p className="error">{error}</p>}
      </label>
      <button type="button" onClick={handleSubmit}>
        Login
      </button>
      <Link to="/auth/registration" className="switch-mode-link">Need to create an account?</Link>
    </StyledForm>
  );
};
