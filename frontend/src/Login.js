import React, { useRef, useState } from "react";
import { Link, Redirect, useHistory } from "react-router-dom";
import { isAuthenticated, fetchData } from "./helpers";
import styled from "styled-components";

const StyledContainer = styled.div`
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  padding: 10em;
  background-color: #f8f9f9;
`;

const StyledLogin = styled.div`
  align-items: flex-start;
  .error {
    color: red;
  }
`;

const getUserToken = async ({ email, password }) => {
  const details = {
    email,
    password
  };

  let token = "";
  let json = {};

  try {
    json = await fetchData(
    `${process.env.REACT_APP_REAL_API}/internal/login`,
    JSON.stringify(details),
    "POST"
  );
  } catch (error) {
    console.log(error);
  }

  if (json.success) {
    token = json.token;
  }

  return token;
};

export const Login = () => {
  const emailRef = useRef(null);
  const passwordRef = useRef(null);
  const [errors, setErrors] = useState({});
  const history = useHistory();

  const handleSubmit = async () => {
    let currentErrors = {};

    const email = emailRef.current.value;
    const password = passwordRef.current.value;

    if (Object.keys(currentErrors).length === 0) {
      try {
        const token = await getUserToken({ email, password });
        if (token.length !== 0) {
          localStorage.setItem("access-token", token);
          return history.push("/");
        } else {
          currentErrors.password = "Please check username or password is entered correctly";
        }
      } catch (error) {
        currentErrors.email = error.message;
      }
    }

    setErrors(currentErrors);
  };

  if (isAuthenticated()) return <Redirect to="/" />;

  return (
    <StyledContainer>
      <StyledLogin>
        <h1>Login</h1>
        <label>
          Username (email):
          <input type="text" ref={emailRef} name="email" />
          <p className="error">{errors.email}</p>
        </label>
        <label>
          Password:
          <input type="password" ref={passwordRef} name="password" />
          <p className="error">{errors.password}</p>
        </label>
        <button onClick={handleSubmit}>Login</button>
        <p><Link to="/registration">Create an account</Link></p>
      </StyledLogin>
    </StyledContainer>
  );
};
