import React from "react";
import styled from "styled-components";
import { Switch } from "react-router-dom";

import { colors } from "./helpers";
import { Login } from "./Login";
import { Logout } from "./Logout";
import { Registration } from "./Registration";
import { ErrorHandledRoute } from "./components/ErrorHandledRoute";

export const AuthRoutes = ({ setUser }) => {
  return (
    <main>
      <StyledAuthRoutes>
          <Switch>
            <ErrorHandledRoute path="/auth/registration">
              <Registration />
            </ErrorHandledRoute>
            <ErrorHandledRoute path="/auth/login">
              <Login setUser={setUser} />
            </ErrorHandledRoute>
            <ErrorHandledRoute path="/auth/logout">
              <Logout />
            </ErrorHandledRoute>
          </Switch>
      </StyledAuthRoutes>
    </main>
  );
};

const StyledAuthRoutes = styled.div`
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  flex: 1;
  margin-top: 5rem;

  & > form {
    & > button {
      margin: 2.5rem 0 0.3rem;
    }

    & > svg {
      width: 65%;
      height: auto;
      margin-bottom: 5rem;
    }

    & > .switch-mode-link {
      margin: 1rem 0;
      color: ${colors.darkGray};
      text-decoration: none;
      &:hover {
        text-decoration: underline;
      }
    }
  }
`;
