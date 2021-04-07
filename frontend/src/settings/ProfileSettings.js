import React, { useState, useRef } from "react";
import { StyledForm, fetchData } from "../helpers";
import { useUser } from "../index";

const editProfile = async ({ avatar, bio }) => {
  return await fetchData(`${process.env.REACT_APP_API}/edit_profile`, JSON.stringify({ avatar, bio }), "PUT");
};

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

    setUser(u => ({...u, bio, avatar}))

    sessionStorage.setItem("access-token", result.token);

    setLoading(false);
  };

  return (
    <StyledForm>
      <label>
        Bio
        <input ref={bioRef} defaultValue={user.bio} />
      </label>
      <label>
        Avatar Link
        <input ref={avatarRef} defaultValue={user.avatar} />
      </label>
      <a href={user.avatar} target="_blank" rel="noopener noreferrer">
        <img src={user.avatar} alt="User Avatar" style={{ width: "10rem", height: "10rem", borderRadius: "7.5rem", margin: "1rem" }} />
      </a>
      <button type="button" onClick={handleConfirm}>
        {loading === true ? "Loading..." : loading === "done" ? "done!" : "Confirm"}
      </button>
    </StyledForm>
  );
};
