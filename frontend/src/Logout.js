import React from "react";
import { Redirect } from "react-router";

export const Logout = () => {
  sessionStorage.removeItem("access-token");  
  sessionStorage.removeItem("username");
  sessionStorage.removeItem("email");

  return <Redirect to="/auth/login" />;
}