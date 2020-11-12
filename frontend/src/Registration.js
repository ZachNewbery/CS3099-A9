import React, { useRef, useState } from "react";
import { Link, Redirect, useHistory } from "react-router-dom";
import { isAuthenticated, fetchData } from "./helpers";
import styled from "styled-components";

const createUser = ({ firstName, lastName, userName, password }) => {
  const user = {
    firstName,
    lastName,
    userName,
    password
  };

  return fetchData(
    `${process.env.REACT_APP_API_URL}/users`,
    JSON.stringify(user),
    "POST"
  );
};

const StyledContainer = styled.div`
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  padding: 10em;
  background-color: #f8f9f9;
`;

const StyledRegistration = styled.div`
  align-items: flex-start;
  .error {
    color: red;
  }
`; 

export const Registration = () => {
  const formRef = useRef(null);
  const [errors, setErrors] = useState({});
  const history = useHistory();

  const handleSubmit = async (e) => {
    e.preventDefault();
    let currentErrors = {};

    let {
      firstName,
      lastName,
      userName,
      password,
      confirmPassword,
    } = formRef.current;

    firstName = firstName.value;
    lastName = lastName.value;
    userName = userName.value;
    password = password.value;
    confirmPassword = confirmPassword.value;

    if (firstName.length < 2) {
      currentErrors.firstName = "First name too short";
    }

    if (lastName.length < 2) {
      currentErrors.lastName = "Last name too short";
    }

    if (userName.length < 2) {
      currentErrors.userName = "Username too short";
    }
    
    if (password.length < 5) {
      currentErrors.password = "Password too short";
      currentErrors.confirmPassword = currentErrors.password;
    }

    if (password !== confirmPassword) {
      currentErrors.confirmPassword = "Passwords don't match";
    }

    if (Object.keys(currentErrors).length === 0) {
      try {
        const user = await createUser({ firstName, lastName, userName, password });
        localStorage.setItem("access-token", "hithere");
        localStorage.setItem("firstName", user.firstName);
        localStorage.setItem("lastName", user.lastName);
        localStorage.setItem("userName", user.userName);
        localStorage.setItem("userId", user.id);
        history.push("/");
      } catch (error) {
        currentErrors.firstName = error.message
      }
    }

    setErrors(currentErrors);
  };

  if (isAuthenticated()) return <Redirect to="/" />;

  return (
    <StyledContainer>
      <StyledRegistration>
        <h1>Registration</h1>
        <form ref={formRef}>
          <label>
            Forename:
            <input type="text" name="firstName" />
            <p class="error">{errors.firstName}</p>
          </label>
          <label>
            Surname:
            <input type="text" name="lastName" />
            <p class="error">{errors.lastName}</p>
          </label>
          <label>
            Username:
            <input type="text" name="userName" />
            <p class="error">{errors.userName}</p>
          </label>
          <label>
            Password:
            <input type="password" name="password" />
            <p class="error">{errors.password}</p>
          </label>
          <label>
            Confirm Password:
            <input type="password" name="confirmPassword" />
            <p class="error">{errors.confirmPassword}</p>
          </label>
          <button onClick={handleSubmit}>Register</button>
        </form>
        <Link to="/login">Already registered?</Link>
      </StyledRegistration>
    </StyledContainer>
  );
};
