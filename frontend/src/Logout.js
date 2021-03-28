import React from "react";
import { Redirect } from "react-router";

export const Logout = () => {
  sessionStorage.removeItem("access-token");  
  sessionStorage.removeItem("user");

  return <Redirect to="/auth/login" />;
}