import React from "react";
import styled from "styled-components";

import { Spinner, Error, colors, fonts } from "../helpers";

export const CTAButton = ({ children, style, disabled = false, isSecondary = false, onClick, isLoading, error }) => (
  <StyledCTAButton style={style} onClick={onClick} secondary={isSecondary} disabled={disabled}>
    <>
      {isLoading && <Spinner />}
      {error || isLoading ? null : children}
      {error && <Error />}
    </>
  </StyledCTAButton>
);

const StyledCTAButton = styled.button`
  cursor: pointer;
  outline: none;
  border: none;
  background: ${colors.blueGradient};
  box-shadow: ${colors.blueInsetShadow};
  padding: 0.5rem;
  width: 100%;
  color: ${colors.white};
  font: inherit;
  font-size: 1rem;
  font-family: ${fonts.accent};
  letter-spacing: 1.5px;
  height: 40px;
  border-radius: 20px;
  display: flex;
  justify-content: center;
  align-items: center;
  transition: all 0.3s;
  &:hover {
    color: white;
    background: ${colors.lightBlueGradient};
  }
  div {
    transform: scale(0.4);
    position: absolute;
    svg {
      margin: 1.2rem;
      circle {
        stroke-opacity: 1;
      }
    }
  }
  img {
    margin: 2px 0 !important;
    height: 1.9rem;
  }
`;
