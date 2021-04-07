import React, { useEffect, useState } from "react";
import styled, { keyframes } from "styled-components";
import { useHistory } from "react-router-dom";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faTimes } from "@fortawesome/free-solid-svg-icons";
import { colors, fonts, Spinner } from "../helpers";
import { CTAButton } from "./CTAButton";
import { ScrollContainer } from "./ScrollContainer";
import { ErrorBoundary } from "./ErrorBoundary";

const fadeIn = keyframes`
  from {
    background: transparent;
  }
  to {
    background: ${colors.modalBackground};
  }
`;

const StyledModalContainer = styled(ScrollContainer)`
  z-index: 200;
  position: fixed;
  display: flex;
  justify-content: center;
  align-items: flex-start;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  width: 100vw;
  height: 100vh;
  animation: ${fadeIn} ${(props) => `${props.animation}s`} forwards;
`;

const StyledModalBackground = styled.div`
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
`;

const StyledExit = styled(FontAwesomeIcon)`
  position: absolute;
  right: 0;
  top: 0;
  cursor: pointer;
  font-size: 1rem;
  padding: 1rem;
  color: ${colors.darkGray};
  box-sizing: content-box;
`;

const StyledModal = styled.div`
  position: relative;
  width: 50rem;
  max-width: 95%;
  min-height: 10rem;
  margin: 10rem auto;
  background: ${colors.pageBackground};
  border: 1px solid ${colors.gray};
  border-radius: 0.8rem;
  display: flex;
  flex-flow: column nowrap;
`;

const StyledActionsContainer = styled.div`
  display: flex;
  margin: 1rem;
  justify-content: flex-end;
  align-items: center;
`;

const StyledChildren = styled.div`
  flex: 1;
  font-size: 1rem;
  padding: 0 1.5rem;
  color: ${colors.text};
`;

const StyledHeader = styled.div`
  padding: 0.5rem 1.25rem 0.25rem;
  h1 {
    color: ${colors.text};
    font-family: ${fonts.accent};
    letter-spacing: 0.7px;
    font-size: 1.5rem;
    font-weight: normal;
    margin: 0;
    text-transform: uppercase;
  }
  h3 {
    margin: 0.5rem 0;
    color: ${colors.lightText};
    font-size: 1rem;
  }
`;

export const Modal = ({
  hide,
  showModal = true,
  children,
  showExit = true,
  animation = 0.3,
  actionText = "Confirm",
  actionCommand,
  showCancel = true,
  isLoading,
  title = "",
  subtitle = "",
  enterKeySubmits = true,
  childrenStyle = {},
  style,
  ...props
}) => {
  useEffect(() => {
    const handleKeyPress = ({ key }) => {
      if (key === "Escape") {
        hide();
      }
      if (key === "Enter" && enterKeySubmits) {
        actionCommand ? actionCommand() : hide();
      }
    };
    window.addEventListener("keydown", handleKeyPress);
    return () => window.removeEventListener("keydown", handleKeyPress);
  }, [hide, actionCommand, enterKeySubmits]);

  if (!showModal) return null;

  return (
    <StyledModalContainer animation={animation} style={style}>
      <StyledModalBackground onClick={hide} />
      <StyledModal {...props}>
        {showExit && <StyledExit icon={faTimes} onClick={hide} />}
        <StyledHeader className="header">
          <h3>{subtitle}</h3>
          <h1>{title}</h1>
        </StyledHeader>
        <StyledChildren className="children" style={childrenStyle}>
          <ErrorBoundary>{children}</ErrorBoundary>
        </StyledChildren>
        {actionCommand && (
          <StyledActionsContainer>
            {showCancel && (
              <CTAButton onClick={hide} isSecondary style={{ marginRight: "1rem" }}>
                Cancel
              </CTAButton>
            )}
            <CTAButton isLoading={isLoading} onClick={() => (actionCommand ? actionCommand() : hide())}>
              {actionText}
            </CTAButton>
          </StyledActionsContainer>
        )}
      </StyledModal>
    </StyledModalContainer>
  );
};

export const LoadingModal = ({ animation = 0.3 }) => {
  const history = useHistory();
  return (
    <Modal animation={animation} showModal={true} hide={() => history.goBack()}>
      <Spinner />
    </Modal>
  );
};

export const AutoHideModal = ({ children, actionCommand, actionText = "Dismiss", ...props }) => {
  const [showModal, setShowModal] = useState(true);
  const hide = () => {
    actionCommand && actionCommand();
    setShowModal(false);
  };
  return (
    <Modal showModal={showModal} hide={hide} actionText={actionText} actionCommand={hide} showCancel={false} showExit={false} {...props}>
      {children}
    </Modal>
  );
};

export const ErrorModal = ({ error, hide }) => {
  const [showModal, setShowModal] = useState(true);
  if (error === "" || error === false) error = "An error occurred, please try again!";

  const handleHide = () => {
    hide && hide();
    setShowModal(false)
  }
  
  return (
    <Modal title="Error" style={{ alignItems: "center" }} showModal={showModal} hide={handleHide} actionText="Okay">
      <p>{error}</p>
    </Modal>
  );
};

export const ConfirmModal = ({ showModal, hide, callback, message = "Are you sure you wish to execute this action?", ...props }) => {
  return (
    <Modal title="Are you sure?" showModal={showModal} hide={hide} actionText="Confirm" actionCommand={callback} {...props}>
      <p>{message}</p>
    </Modal>
  );
};
