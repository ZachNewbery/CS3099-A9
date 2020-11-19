import React from "react";
import { Redirect } from "react-router";

export const Logout = () => {
  localStorage.removeItem("access-token");
  //localStorage.removeItem("firstName");
  //localStorage.removeItem("lastName");
  //localStorage.removeItem("userName");
  //localStorage.removeItem("userId");
  localStorage.removeItem("username");
  localStorage.removeItem("email");

  return <Redirect to="/login" />;
}