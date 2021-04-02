import React from "react";
import styled from "styled-components";
import { colors, fonts } from "../helpers";

export const Tooltip = ({ text, ...props }) => {
  return (
    <StyledTooltip {...props}>
      <p>{text}</p>
    </StyledTooltip>
  );
};

const StyledTooltip = styled.div`
  width: 8rem;
  background: ${colors.blueGradient};
  color: ${colors.white};
  font-family: ${fonts.default};
  box-shadow: ${colors.blueInsetShadow}, 0 10px 25px -10px rgb(9 98 189 / 64%), 0 40px 70px -15px rgb(32 89 234 / 79%);
  text-align: center;
  border-radius: 0.3rem;
  padding: 0.3rem 0.5rem;
  position: absolute;
  z-index: 1;
  left: 100%;
  top: 0;
  margin: 0 1rem;

  &::after {
    content: "";
    position: absolute;
    right: 88.8%;
    top: 11%;
    bottom: 0;
    height: 78%;
    aspect-ratio: 1 / 1;
    background: red;
    clip-path: polygon(0% 0%, 0% 100%, 100% 0%);
    border-radius: 0.3rem;
    transform: rotate(-45deg);
    background: #0d0ab7;
    box-shadow: ${colors.blueInsetShadow};
  }

  p {
    margin: 0;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
`;
