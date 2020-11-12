import React from "react";
import { Redirect } from "react-router";

export const Logout = () => {
  localStorage.removeItem("access-token");

  return <Redirect to="/login" />;
}