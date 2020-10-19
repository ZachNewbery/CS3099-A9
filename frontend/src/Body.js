import React from "react";
import styled from "styled-components";

const StyledPost = styled.div`
  background: ${props => props.colour};
  p {
    color: blue;
  }
`;

const posts = [
  {
    title: "My Post",
    description: "hi there"
  },
  {
    title: "My Post 2",
    description: "hi there 2"
  },
  {
    title: "My Post 3",
    description: "hi there 3"
  },
]

export const Body = () => {
  return (
    <div>
      {posts.map((post, i) => (
         <Post {...post} key={i} />
      ))}
    </div>
  )
}

export const Post = ({ title, description }) => (
  <StyledPost colour="orange">
    <h1>{title}</h1>
    <p>{description}</p>
  </StyledPost>
)