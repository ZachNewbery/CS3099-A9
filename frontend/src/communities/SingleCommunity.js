import React, { useState, useEffect, useContext } from "react";
import styled from "styled-components";
import { useAsync } from "react-async";

import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faPencilAlt, faTrash } from "@fortawesome/free-solid-svg-icons";
import { fetchData, Spinner, Error, colors, fonts } from "../helpers";
import { InstanceContext, CommunityContext } from "../App";

import { useUser } from "../index";
import { EditCommunity } from "./EditCommunity";

const StyledContainer = styled.div`
  display: flex;
  flex-flow: column nowrap;
  background: ${colors.blueGradient};
  box-shadow: ${colors.blueInsetShadow};
  border-radius: 0.6rem;
  margin-bottom: 1rem;
  padding: 0 1rem;

  & > h1 {
    margin: 0;
    font-family: ${fonts.accent};
    color: ${colors.white};
    font-weight: normal;
    font-size: 1.25rem;
    letter-spacing: 0.5px;
    display: -webkit-box;
    -webkit-line-clamp: 3;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }

  & > .actions {
    display: flex;
    padding: 0.75rem 0 0;
    & > p {
      margin: 0;
      font-size: 0.8rem;
      color: ${colors.softWhite};
      font-family: ${fonts.accent};
      letter-spacing: 0.5px;
      flex: 1;
    }
    & > svg {
      color: ${colors.softWhite};
      transition: all 0.3s;
      font-size: 0.75rem;
      margin-left: 0.5rem;
      cursor: pointer;
      &:hover {
        color: ${colors.white};
      }
    }
  }

  & > .content {
    & > p {
      color: ${colors.white};
      margin: 0.5rem 0 1rem;
      font-size: 1rem;
      display: -webkit-box;
      -webkit-line-clamp: 6;
      -webkit-box-orient: vertical;
      overflow: hidden;
    }
  }
`;

const fetchCommunity = async ({ community, instance }) => {
  const url = new URL(`${process.env.REACT_APP_API}/communities/${community}`);
  const appendParam = (key, value) => value && url.searchParams.append(key, value);
  appendParam("host", instance);
  return fetchData(url);
};

const deleteCommunity = async ({ community, instance }) => {
  const url = new URL(`${process.env.REACT_APP_API}/communities/${community}`);
  const appendParam = (key, value) => value && url.searchParams.append(key, value);
  appendParam("host", instance);
  return await fetchData(url, null, "DELETE");
};

export const SingleCommunity = ({ communities, refresh }) => {
  const [selectedCommunity, setSelectedCommunity] = useState(null);
  const [showCommunity, setShowCommunity] = useState(false);

  const { community, setCommunity } = useContext(CommunityContext);
  const { instance } = useContext(InstanceContext);

  useEffect(() => {
    if (!community && communities?.length) {
      setCommunity(communities[0].id);
    }
  }, [community, communities, setCommunity]);

  useEffect(() => {
    const loadCommunity = async () => {
      const data = await fetchCommunity({ community, instance });
      setSelectedCommunity(data);
    };
    community && loadCommunity();
  }, [community, instance]);

  const handleShowCommunity = () => setShowCommunity(true);
  const handleHideCommunity = () => setShowCommunity(false);

  const handleEdit = () => handleShowCommunity();
  const handleDelete = async (e) => {
    e.preventDefault();
    await deleteCommunity({ community, instance });
    setCommunity(null);
    refresh();
  };

  return (
    <>
      {selectedCommunity && (
        <EditCommunity
          show={showCommunity}
          hide={handleHideCommunity}
          refresh={refresh}
          id={community}
          initialTitle={selectedCommunity?.title}
          initialDescription={selectedCommunity?.description}
          key={selectedCommunity?.title}
        />
      )}
      <Community communities={communities} community={selectedCommunity} handleEdit={handleEdit} handleDelete={handleDelete} />
    </>
  );
};

export const Community = ({ community, communities, handleDelete, handleEdit }) => {
  const user = useUser();

  if (!community && !communities.length)
    return (
      <StyledContainer>
        <div className="content">
          <p style={{ margin: "1rem 0" }}>This instance has no communities yet!</p>
        </div>
      </StyledContainer>
    );

  if (!community) return <Spinner />;

  const isAdmin = community.admins.find((admin) => admin.id.toLowerCase() === user.id.toLowerCase() && admin.host.toLowerCase() === user.host.toLowerCase());

  return (
    <StyledContainer>
      <div className="actions">
        <p>{`${community.admins.length} ${community.admins.length === 1 ? "admin" : "admins"}`}</p>
        {isAdmin && (
          <>
            <FontAwesomeIcon onClick={handleEdit} icon={faPencilAlt} />
            <FontAwesomeIcon onClick={handleDelete} icon={faTrash} />
          </>
        )}
      </div>
      <h1 title={community.title}>{community.title}</h1>
      <div className="content">
        <p title={community.description}>{community.description}</p>
      </div>
    </StyledContainer>
  );
};
