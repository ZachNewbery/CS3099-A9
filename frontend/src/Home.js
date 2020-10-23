import React from "react";
import { Redirect, useHistory } from "react-router-dom";
import styled from "styled-components";
import { isAuthenticated } from "./helpers";

const StyledContainer = styled.div`
  display: flex;
`;

export const Home = () => {
  const history = useHistory();

  if (!isAuthenticated()) return <Redirect to='/login' />;

  return (
    <StyledContainer>
      <button onClick={() => history.push("/logout")}>Logout</button>
    </StyledContainer>
  )
}