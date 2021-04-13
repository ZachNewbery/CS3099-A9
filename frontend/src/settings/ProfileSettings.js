import React, { useState, useRef } from "react";
import styled from "styled-components"; 
import { StyledForm, fetchData } from "../helpers";
import { useUser } from "../index";
import { Profile } from "../components/Profile";

const editProfile = async ({ avatar, bio }) => {
  return await fetchData(`${process.env.REACT_APP_API}/edit_profile`, JSON.stringify({ avatar, bio }), "PUT");
};

const StyledProfile = styled(Profile)`
  width: 5rem;
  height: 5rem;
  & > img {
    height: 5rem;
  }
`;

export const ProfileSettings = () => {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);

  const { setUser, ...user } = useUser();

  const bioRef = useRef(null);
  const avatarRef = useRef(null);

  const handleConfirm = async () => {
    const bio = bioRef.current.value;
    const avatar = avatarRef.current.value;

    setLoading(true);

    const result = await editProfile({ avatar, bio });

    setUser((u) => ({ ...u, about: bio, avatarUrl: avatar }));

    sessionStorage.setItem("access-token", result.token);

    setLoading(false);
  };

  return (
    <StyledForm>
      <a href={user.avatarUrl} target="_blank" rel="noopener noreferrer">
        <StyledProfile hasClickthrough={false} user={user} />
      </a>
      <label>
        Avatar Link
        <input ref={avatarRef} defaultValue={user.avatarUrl} />
      </label>
      <label>
        Bio
        <textarea ref={bioRef} defaultValue={user.about} />
      </label>
      <button type="button" onClick={handleConfirm}>
        {loading === true ? "Loading..." : loading === "done" ? "done!" : "Confirm"}
      </button>
    </StyledForm>
  );
};
