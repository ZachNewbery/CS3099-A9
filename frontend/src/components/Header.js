import React from "react";
import styled from "styled-components";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faSignOutAlt, faCog } from "@fortawesome/free-solid-svg-icons";
import { Link } from "react-router-dom";

import { Logo } from "../assets/Logo";
import { fonts, colors } from "../helpers";

const StyledHeader = styled.header`
  width: 100%;
  height: 5rem;
  padding: 1rem;
  display: flex;
  border-bottom: 1px solid ${colors.lightGray};
  justify-content: space-between;
  background: ${colors.veryLightGray};

  & > .logo-container {
    display: flex;
    align-items: center;
    & > svg {
      height: 100%;
      width: auto;
    }
    & > .logo-text {
      margin: -0.45rem 1rem 0;
      & > h3 {
        margin: 0;
        font-size: 1.6rem;
        font-family: ${fonts.accent};
        font-weight: normal;
      }
      & > p {
        margin: 0;
        font-size: 1.1rem;
        font-family: ${fonts.accent};
        font-weight: normal;
        color: rgba(0, 0, 0, 0.6);
        letter-spacing: 1.5px;
        word-spacing: 6.1px;
        line-height: 0.5;
      }
    }
  }

  & > .actions-container {
    display: flex;
    align-items: center;

    & > .action {
      text-decoration: none;
      cursor: pointer;
      display: flex;
      justify-content: center;
      align-items: center;
      padding: 0.4rem 0.8rem;
      border: 1px solid ${colors.lightGray};
      border-radius: 0.25rem;
      margin-left: 0.5rem;
      background: rgba(255, 255, 255, 0.6);
      font-family: ${fonts.accent};
      letter-spacing: 1.4px;
      font-size: 1rem;
      box-shadow: 0 1.5px 1px rgba(0, 0, 0, 0.3);
      transition: all 0.3s;
      color: inherit !important;
      &:hover {
        background: rgba(255, 255, 255, 0.4);
      }
      &:active {
        background: rgba(255, 255, 255, 0.2);
      }
      & > svg {
        margin-right: 0.5rem;
        color: ${colors.darkGray};
      }
    }
  }
`;

export const Header = () => {
  return (
    <StyledHeader>
      <div className="logo-container">
        <Logo />
        <div className="logo-text">
          <h3>Fediversity</h3>
          <p>To The Moon!</p>
        </div>
      </div>
      <div className="actions-container">
        <Link to="/logout" className="action">
          <FontAwesomeIcon icon={faSignOutAlt} />
          Logout
        </Link>
        <Link to="#" className="action">
          <FontAwesomeIcon icon={faCog} />
          Settings
        </Link>
      </div>
    </StyledHeader>
  );
};
