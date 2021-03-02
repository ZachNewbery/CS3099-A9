import React from "react";
import styled from "styled-components";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

import { colors, fonts } from "../helpers";
import { TABS } from "./index";

export const Tabs = ({ tab, setTab }) => {
  return (
    <StyledTabs>
      {Object.entries(TABS).map(([key, value]) => (
        <Tab key={key} tab={value} selectedTab={tab} setTab={setTab} />
      ))}
    </StyledTabs>
  );
};

const Tab = ({ tab, selectedTab, setTab }) => {
  const handleClick = () => setTab(tab);

  return (
    <div onClick={handleClick} className={tab.text === selectedTab.text ? "active" : ""}>
      <FontAwesomeIcon icon={tab.icon} />
      <p>{tab.text}</p>
    </div>
  );
};

const StyledTabs = styled.div`
  display: flex;
  flex-flow: column nowrap;
  width: 12rem;
  padding: 0.5rem 0;
  margin: 1rem 0;

  & > div {
    cursor: pointer;
    display: flex;
    justify-content: flex-start;
    align-items: center;
    margin-bottom: 0.5rem;
    padding: 0 1rem;
    border-radius: 1rem;
    font-family: ${fonts.accent};
    letter-spacing: 0.6px;
    font-size: 0.8rem;
    height: 2rem;
    color: ${colors.text} !important;
    background: ${colors.pageBackground};
    & > svg {
      margin-right: 0.5rem;
      color: ${colors.lightText};
    }

    &.active {
      color: ${colors.white} !important;
      background: ${colors.blueGradient};
      box-shadow: ${colors.blueInsetShadow};
      & > svg {
        color: ${colors.white};
      }
    }
  }
`;
