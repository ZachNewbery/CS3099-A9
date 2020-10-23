import React, { useRef, useState } from "react";
import { Link, Redirect, useHistory } from "react-router-dom";
import { isAuthenticated } from "./helpers";
import styled from "styled-components";

const StyledContainer = styled.div`
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
`;

export const Login = () => {
  const usernameRef = useRef(null);
  const passwordRef = useRef(null);
  const [errors, setErrors] = useState({});
  const history = useHistory();

  const handleSubmit = () => {
    setErrors({});

    const username = usernameRef.current.value;
    const password = passwordRef.current.value;

    if (Object.keys(errors).length === 0) {
      localStorage.setItem('access-token', 'hithere');
      history.push('/');
    }

    console.log(username, password);
  }

  if (isAuthenticated()) return <Redirect to='/' />;

  return (
    <StyledContainer>
      <label>
        Username:
        <input type='text' ref={usernameRef} name='username' />
        <p>{errors.username}</p>
      </label>      
      <label>
        Password:
        <input type='password' ref={passwordRef} name='password' />  
        <p>{errors.password}</p>  
      </label>
      <button onClick={handleSubmit}>Login</button>
      <Link to='/registration'>Create an account</Link> 
    </StyledContainer>
  )
}