import React, { useState } from "react";
import styled from "styled-components";
import { faUser, faCog, faLock } from "@fortawesome/free-solid-svg-icons";

import { Modal } from "../components/Modal";
import { Tabs } from "../components/Tabs";
import { colors } from "../helpers";

import { PasswordReset } from "./PasswordReset";
import { ProfileSettings } from "./ProfileSettings";

export const TABS = {
  PROFILE: { text: "User Profile", icon: faUser },
  PASSWORD: { text: "Password Reset", icon: faLock },
};

export const Settings = ({ show, hide }) => {
  const [tab, setTab] = useState(TABS.PROFILE);

  if (!show) return null;

  const renderContent = () => {
    switch (tab.text) {
      case TABS.PROFILE.text:
        return <ProfileSettings />;
      case TABS.PASSWORD.text:
        return <PasswordReset />;
      default:
        return null;
    }
  };

  return (
    <Modal title="Settings" showModal={show} hide={hide} childrenStyle={{ display: "flex", height: "30rem" }}>
      <Tabs tab={tab} setTab={setTab} tabs={TABS} />
      <StyledContent>{renderContent()}</StyledContent>
    </Modal>
  );
};

const StyledContent = styled.div`
  flex: 1;
  margin: 0.5rem 0 1rem 1rem;
  padding: 1rem;
  border-left: 1px solid ${colors.gray};
  display: flex;
  flex-flow: column nowrap;
  align-items: center;

  input {
    margin: 0.3rem 0;
    padding: 0.45rem;
    border: 1px solid ${colors.lightGray};
    background: none !important;
    border-radius: 0.3rem;
    font: inherit;
    font-size: 1rem;
  }
`;
