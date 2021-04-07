import React from "react";
import { Route } from "react-router-dom";
import { ErrorBoundary } from "./ErrorBoundary";

export const ErrorHandledRoute = ({ path, children }) => (
  <Route path={path}>
    <ErrorBoundary withMessage={true}>
      {children}
    </ErrorBoundary>
  </Route>
);