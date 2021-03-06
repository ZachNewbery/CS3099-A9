import React, { useRef, useState } from "react";
import { Link, Redirect, useHistory } from "react-router-dom";
import { isAuthenticated, fetchData, StyledForm } from "./helpers";
import { Logo } from "./assets/Logo";

const getUserToken = async ({ email, password }) => {
  return await fetchData(`${process.env.REACT_APP_API}/login`, JSON.stringify({ email, password }), "POST");
};

export const Login = ({ setUser }) => {
  const emailRef = useRef(null);
  const passwordRef = useRef(null);
  const [error, setError] = useState();
  const history = useHistory();

  const handleSubmit = async () => {
    const email = emailRef.current.value;
    const password = passwordRef.current.value;

    try {
      const user = await getUserToken({ email, password });
      const { token } = user;
      sessionStorage.setItem("access-token", token);
      const normalizedUser = {
        avatarUrl: user.avatar,
        about: user.bio,
        id: user.username,
        host: user.host
      }
      setUser(normalizedUser);
      return history.push("/");
    } catch (error) {
      setError("Please check email or password is entered correctly.");
    }
  };

  const handleKeyDown = async (e) => {
    if (e.key === "Enter") {
      await handleSubmit();
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
        <input type="password" ref={passwordRef} name="password" onKeyDown={handleKeyDown} />
        {error && <p className="error">{error}</p>}
      </label>
      <button type="button" onClick={handleSubmit}>
        Login
      </button>
      <Link to="/auth/registration" className="switch-mode-link">
        Need to create an account?
      </Link>
    </StyledForm>
  );
};
