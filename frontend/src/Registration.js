import React, { useRef, useState } from "react";
import { Link, Redirect, useHistory } from "react-router-dom";
import { isAuthenticated } from "./helpers";
import styled from "styled-components";

const StyledContainer = styled.div`
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;

  form {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
  }
`;

export const Registration = () => {
  const formRef = useRef(null);
  const [errors, setErrors] = useState({});
  const history = useHistory();

  const handleSubmit = e => {
    e.preventDefault();
    setErrors({});

    let { firstName, lastName, userName, password, confirmPassword } = formRef.current;
    
    firstName = firstName.value;
    lastName = lastName.value;
    userName = userName.value;
    password = password.value;
    confirmPassword = confirmPassword.value;

    if (password !== confirmPassword)  {
      setErrors(currentErrors => ({ ...currentErrors, confirmPassword: 'Passwords don\'t match' }));
    }

    if (Object.keys(errors).length === 0) {
      localStorage.setItem('access-token', 'hithere');
      history.push('/');
    }

    console.log({ firstName, lastName, userName, password, confirmPassword });
  }

  if (isAuthenticated()) return <Redirect to='/' />;

  return (
    <StyledContainer>
      <form ref={formRef}>
        <label>
          Forename:
          <input type='text' name='firstName' />
        </label>
        <label>
          Surname:
          <input type='text' name='lastName' />
        </label>
        <label>
          Username:
          <input type='text' name='userName' />
        </label>      
        <label>
          Password:
          <input type='password' name='password' />    
        </label>
        <label>
          Confirm Password:
          <input type='password' name='confirmPassword' />   
          <p>{errors.confirmPassword}</p>
        </label>
        <button onClick={handleSubmit}>Register</button>
      </form>
      <Link to='/login'>Already registered?</Link>
    </StyledContainer>
  )
}