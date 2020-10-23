import React from "react";
import { Redirect } from "react-router-dom";
import { isAuthenticated } from "./helpers";

export const Home = () => {
  if (!isAuthenticated()) return <Redirect to='/login' />
  return <h1>Home</h1>
}