import React, { useRef, useState } from "react";
import styled from "styled-components";
import { Link, Redirect, useHistory } from "react-router-dom";
import { isAuthenticated, fetchData, colors, fonts } from "./helpers";
import { Logo } from "./assets/Logo";

export const StyledAuth = styled.div`
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  flex: 1;

  & > form {
    align-items: flex-start;
    display: flex;
    flex-flow: column nowrap;
    justify-content: center;
    align-items: center;
    width: 20rem;

    & > label {
      display: flex;
      flex-direction: column;
      font-size: 0.9rem;
      width: 100%;
      margin-bottom: 1.25rem;
      position: relative;

      & > input {
        margin: 0.3rem 0;
        padding: 0.45rem;
        border: 1px solid ${colors.lightGray};
        border-radius: 0.3rem;
        font: inherit;
        font-size: 1rem;
      }

      & > .error {
        position: absolute;
        color: red;
        margin: 0.1rem 0.25rem;
        font-size: 0.8rem;
        text-align: center;
        top: 100%;
        left: 0;
        right: 0;
      }
    }

    & > svg {
      width: 65%;
      height: auto;
      margin-bottom: 5rem;
    }

    & > button {
      cursor: pointer;
      outline: none;
      border: none;
      background: ${colors.blue};
      padding: 0.5rem;
      width: 100%;
      margin: 2.5rem 0 0.3rem;
      color: white;
      font: inherit;
      font-size: 1.15rem;
      font-family: ${fonts.accent};
      letter-spacing: 1.5px;
      height: 40px;
      border-radius: 20px;
      display: flex;
      justify-content: center;
      align-items: center;
      transition: all 0.3s;
      &:hover {
        background: ${colors.lightBlue};
      }
    }
  }
`;

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
    <form onChange={() => setError(null)}>
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
      <p>
        <Link to="/registration">Create an account</Link>
      </p>
    </form>
  );
};
