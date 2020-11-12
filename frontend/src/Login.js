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

const authenticate = async ({ userName, password }) => {
  const [user] = await fetchData(
    `${process.env.REACT_APP_API_URL}/users?userName=${userName}`
  );

  return user && user.password === password;
};

export const Login = () => {
  const userNameRef = useRef(null);
  const passwordRef = useRef(null);
  const [errors, setErrors] = useState({});
  const history = useHistory();

  const handleSubmit = async () => {
    let currentErrors = {};

    const userName = userNameRef.current.value;
    const password = passwordRef.current.value;

    if (Object.keys(currentErrors).length === 0) {
      try {
        const isAuthenticated = await authenticate({ userName, password });
        if (isAuthenticated) {
          localStorage.setItem("access-token", "hithere");
          return history.push("/");
        } else {
          currentErrors.password = "Please check username or password is entered correctly";
        }
      } catch (error) {
        currentErrors.userName = error.message;
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
          Username:
          <input type="text" ref={userNameRef} name="userName" />
          <p class="error">{errors.userName}</p>
        </label>
        <label>
          Password:
          <input type="password" ref={passwordRef} name="password" />
          <p class="error">{errors.password}</p>
        </label>
        <button onClick={handleSubmit}>Login</button>
        <p><Link to="/registration">Create an account</Link></p>
      </StyledLogin>
    </StyledContainer>
  );
};
