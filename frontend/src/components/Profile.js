import React, { useState, useLayoutEffect } from "react";
import styled from "styled-components";

import { colors, fonts } from "../helpers";

const StyledProfilePicture = styled.div`
  width: 2.5rem;
  height: 2.5rem;
  border-radius: 1.5rem;
  overflow: hidden;
  display: flex;
  justify-content: center;
  align-items: center;
  position: relative;
  cursor: pointer;
  & > img {
    height: 2.5rem;
    width: auto;
  }
`;

const StyledModalContainer = styled.div`
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  position: fixed;
  background: transparent
  z-index: 98;
`;

const StyledModal = styled.div`
  position: absolute;
  top: 100%;
  left: 0%;
  box-shadow: ${colors.blueInsetShadow}, 0 5px 12px -5px rgb(9 98 189 / 50%), 0 20px 35px -7px rgb(32 89 234 / 65%);
  background: ${colors.blueGradient};
  border-radius: 0.8rem;
  margin: 0.5rem 0;
  padding: 0.75rem;
  z-index: 99;
  color: ${colors.white};
  width: 10rem;

  & > h3 {
    margin: 0;
    font-family: ${fonts.accent};
    font-size: 1rem;
    letter-spacing: 0.5px
  }

  & > p {
    margin: 0;
    font-size: 0.8rem;
    color: ${colors.softWhite};
  }
`;

export const Profile = ({ user }) => {
  const [showModal, setShowModal] = useState(false);

  return (
    <>
      <div style={{ position: "relative" }} className="profile-picture">
        <StyledProfilePicture onClick={() => setShowModal(true)}>
          <img
            alt="profile"
            src={
              user.avatar || `https://eu.ui-avatars.com/api/?rounded=true&bold=true&background=0061ff&color=ffffff&uppercase=true&format=svg&name=${user.id}`
            }
          />
        </StyledProfilePicture>
        {showModal && <ProfileModal user={user} />}
      </div>
      {showModal && <StyledModalContainer onClick={() => setShowModal(false)} />}
    </>
  );
};

const ProfileModal = ({ user }) => {
  return (
    <StyledModal>
      <h3>{user.username}</h3>
      <p>{user.bio}</p>
    </StyledModal>
  );
};
