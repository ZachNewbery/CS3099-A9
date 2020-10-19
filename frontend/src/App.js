import React from "react";
import { Body } from "./Body";

export const App = ({ text }) => {
  return (
    <>
      <Heading text={text} />
      <Body />
    </>
  )
}

export const Heading = ({ text = "heading" }) => {
  return <h1>{text}</h1>
}

