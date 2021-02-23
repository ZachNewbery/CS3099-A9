import React from "react";
import styled, { keyframes } from "styled-components";
import { colors } from "../helpers";

const StyledMessage = styled.p`
  color: ${colors.gray};
  text-align: center;
  line-height: 1.5;
  font-size: 1.5rem;
  white-space: pre-wrap;
  font-size: 1rem;
`;

const fadeIn = keyframes`
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
`;

const StyledContainer = styled.div`
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  opacity: 0;
  animation: ${fadeIn} 0.6s 0.6s ease-out forwards;
`;

const StyledError = styled.svg`
  fill: ${colors.darkGray};
`;

export const Error = ({ message }) => {
  return (
    <StyledContainer>
      <ErrorSvg style={{ margin: "2rem auto 0" }} />
      <StyledMessage>{message?.error?.message || message?.message || JSON.stringify(message) || "An error occurred"}</StyledMessage>
    </StyledContainer>
  );
};

const ErrorSvg = (props) => {
  return (
    <StyledError {...props} xmlns="http://www.w3.org/2000/svg" viewBox="0 0 486 486" height="50" width="50">
      <g>
        <path
          d="M243.225,333.382c-13.6,0-25,11.4-25,25s11.4,25,25,25c13.1,0,25-11.4,24.4-24.4
			C268.225,344.682,256.925,333.382,243.225,333.382z"
        />
        <path
          d="M474.625,421.982c15.7-27.1,15.8-59.4,0.2-86.4l-156.6-271.2c-15.5-27.3-43.5-43.5-74.9-43.5s-59.4,16.3-74.9,43.4
			l-156.8,271.5c-15.6,27.3-15.5,59.8,0.3,86.9c15.6,26.8,43.5,42.9,74.7,42.9h312.8
			C430.725,465.582,458.825,449.282,474.625,421.982z M440.625,402.382c-8.7,15-24.1,23.9-41.3,23.9h-312.8
			c-17,0-32.3-8.7-40.8-23.4c-8.6-14.9-8.7-32.7-0.1-47.7l156.8-271.4c8.5-14.9,23.7-23.7,40.9-23.7c17.1,0,32.4,8.9,40.9,23.8
			l156.7,271.4C449.325,369.882,449.225,387.482,440.625,402.382z"
        />
        <path
          d="M237.025,157.882c-11.9,3.4-19.3,14.2-19.3,27.3c0.6,7.9,1.1,15.9,1.7,23.8c1.7,30.1,3.4,59.6,5.1,89.7
			c0.6,10.2,8.5,17.6,18.7,17.6c10.2,0,18.2-7.9,18.7-18.2c0-6.2,0-11.9,0.6-18.2c1.1-19.3,2.3-38.6,3.4-57.9
			c0.6-12.5,1.7-25,2.3-37.5c0-4.5-0.6-8.5-2.3-12.5C260.825,160.782,248.925,155.082,237.025,157.882z"
        />
      </g>
    </StyledError>
  );
};
