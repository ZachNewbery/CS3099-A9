import React, { useState, useContext } from "react";
import styled from "styled-components";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faSignOutAlt, faCog } from "@fortawesome/free-solid-svg-icons";
import { Link } from "react-router-dom";

import { InstanceContext, CommunityContext } from "../App";
import { Settings } from "../settings";
import { Logo } from "../assets/Logo";
import { fonts, colors } from "../helpers";

const StyledHeader = styled.header`
  width: 100%;
  height: 5rem;
  padding: 1rem;
  display: flex;
  background: ${colors.pageBackground};
  position: relative;
  justify-content: space-between;
  z-index: 10;
  box-shadow: 0 10px 70px -30px rgb(6 27 225 / 40%);
  position: sticky;
  top: 0;

  & > .logo-container {
    text-decoration: none;
    display: flex;
    align-items: center;
    width: 12rem;
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
        color: ${colors.text};
      }
      & > p {
        margin: 0;
        font-size: 1.1rem;
        font-family: ${fonts.accent};
        font-weight: normal;
        color: ${colors.lightText};
        letter-spacing: 1.5px;
        word-spacing: 6.1px;
        line-height: 0.5;
      }
    }
  }

  & > .content {
    z-index: -1;
    position: absolute;
    left: 0;
    top: 1.25rem;
    right: 0;
    bottom: 0;
    display: flex;
    justify-content: center;
    align-items: flex-start;
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
      padding: 0 1.25rem;
      border-radius: 1.25rem;
      margin-left: 0.5rem;
      background: rgba(255, 255, 255, 0.6);
      font-family: ${fonts.accent};
      letter-spacing: 1.4px;
      font-size: 1rem;
      height: 2.5rem;
      color: ${colors.white} !important;
      background: ${colors.blueGradient};
      box-shadow: ${colors.blueInsetShadow};
      transition: all 0.3s;
      &:hover {
        color: white !important;
        background: ${colors.lightBlueGradient};
      }
      & > svg {
        margin-right: 0.5rem;
        color: ${colors.white};
      }
    }
  }
`;

export const Header = ({ children }) => {
  const [showSettings, setShowSettings] = useState(false);

  const { setInstance, INTERNAL_INSTANCE } = useContext(InstanceContext);
  const { setCommunity } = useContext(CommunityContext);

  const handleShowSettings = () => setShowSettings(true);
  const handleHideSettings = () => setShowSettings(false);

  const handleClick = () => {
    setInstance(INTERNAL_INSTANCE);
    setCommunity(null);
  };

  return (
    <>
      <Settings show={showSettings} hide={handleHideSettings} />
      <StyledHeader>
        <Link to="/" className="logo-container" onClick={handleClick}>
          <Logo />
          <div className="logo-text">
            <h3>Fediversity</h3>
            <p>To The Moon!</p>
          </div>
        </Link>
        <div className="content">{children}</div>
        <div className="actions-container">
          <Link to="#" className="action" onClick={handleShowSettings}>
            <FontAwesomeIcon icon={faCog} />
            Settings
          </Link>
          <Link to="/auth/logout" className="action">
            <FontAwesomeIcon icon={faSignOutAlt} />
            Logout
          </Link>
        </div>
      </StyledHeader>
    </>
  );
};
