import React, { useRef, useState } from "react";
import { Link, Redirect, useHistory } from "react-router-dom";
import { isAuthenticated, fetchData, StyledForm } from "./helpers";
import { Logo } from "./assets/Logo";

const createUser = ({ username, email, password }) => {
  return fetchData(`${process.env.REACT_APP_API}/new_user`, JSON.stringify({ username, email, password }), "POST");
};

export const Registration = () => {
  const formRef = useRef(null);
  const [errors, setErrors] = useState({});
  const history = useHistory();

  const handleSubmit = async (e) => {
    e.preventDefault();
    let currentErrors = {};

    let { username, email, password, confirmPassword } = formRef.current;

    username = username.value;
    email = email.value;
    password = password.value;
    confirmPassword = confirmPassword.value;

    if (username.length < 2) {
      currentErrors.username = "Username too short";
    }

    if (email.length < 6) {
      // TODO Regex?
      currentErrors.email = "Email invalid";
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
        const user = await createUser({ username, email, password });
        console.log(user);
        sessionStorage.setItem("username", user.username);
        sessionStorage.setItem("host", user.host);
        history.push("/login");
      } catch (error) {
        currentErrors.firstName = error.message;
      }
    }

    setErrors(currentErrors);
  };

  if (isAuthenticated()) return <Redirect to="/" />;

  return (
    <StyledForm ref={formRef}>
      <Logo />
      <label>
        Username
        <input type="text" name="username" />
        <p className="error">{errors.username}</p>
      </label>
      <label>
        Email
        <input type="text" name="email" />
        <p className="error">{errors.email}</p>
      </label>
      <label>
        Password
        <input type="password" name="password" />
        <p className="error">{errors.password}</p>
      </label>
      <label>
        Confirm Password
        <input type="password" name="confirmPassword" />
        <p className="error">{errors.confirmPassword}</p>
      </label>
      <button onClick={handleSubmit}>Register</button>
      <Link to="/auth/login" className="switch-mode-link">Already registered?</Link>
    </StyledForm>
  );
};
