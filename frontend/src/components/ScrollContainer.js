import React from "react";
import styled from "styled-components";
import { colors } from "../helpers";

const StyledScroll = styled.div`
  overflow-y: ${(props) => props.scrolly};
  overflow-x: ${(props) => props.scrollx};
  &::-webkit-scrollbar {
    width: 6px;
    height: 6px;
  }

  &::-webkit-scrollbar-corner {
    background: ${colors.pageBackground};
  }

  &::-webkit-scrollbar-thumb {
    border-radius: 3px;
    background: ${props => props.scrollcolor || colors.scrollbarColor};
    transition: background 0.3s;
    &:hover {
      background: ${props => props.scrollhover || colors.scrollbarHover};
    }
  }
`;

export const ScrollContainer = ({ scrollX = "unset", scrollY = "auto", children, ...props }) => {
  return (
    <StyledScroll scrollx={scrollX} scrolly={scrollY} {...props} className={`scroll ${props.className}`}>
      {children}
    </StyledScroll>
  );
};
