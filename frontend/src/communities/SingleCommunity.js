import React, { useState } from "react";
import styled from "styled-components";
import { useAsync } from "react-async";

import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faPencilAlt, faTrash } from "@fortawesome/free-solid-svg-icons";
import { fetchData, Spinner, Error, colors, fonts } from "../helpers";

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

const fetchCommunity = async ({ id, host }) => {
  const hostParam = host ? `?host=${host}` : "";
  return await fetchData(`${process.env.REACT_APP_API}/communities/${id}${hostParam}`);
};

const deleteCommunity = async ({ id }) => {
  return await fetchData(`${process.env.REACT_APP_API}/communities/${id}`, null, "DELETE");
};

export const SingleCommunity = ({ id, host, refresh }) => {
  const [showCommunity, setShowCommunity] = useState(false);

  const { data, isLoading, error } = useAsync(fetchCommunity, { id, host });

  const user = useUser();

  const handleShowCommunity = () => setShowCommunity(true);
  const handleHideCommunity = () => setShowCommunity(false);

  const handleEdit = () => handleShowCommunity();
  const handleDelete = async (e) => {
    e.preventDefault();
    await deleteCommunity({ id });
    refresh();
  };

  const renderCommunity = () => {
    if (isLoading) return <Spinner />;
    if (error) return <Error message={error} />;

    const isAdmin = data.admins.find((admin) => admin.id.toLowerCase() === user.username.toLowerCase() && admin.host.toLowerCase() === user.host.toLowerCase());

    return (
      <>
        <div className="actions">
          <p>{`${data.admins.length} ${data.admins.length === 1 ? "admin" : "admins"}`}</p>
          {isAdmin && (
            <>
              <FontAwesomeIcon onClick={handleEdit} icon={faPencilAlt} />
              <FontAwesomeIcon onClick={handleDelete} icon={faTrash} />
            </>
          )}
        </div>
        <h1 title={id}>{id}</h1>
        <div className="content">
          <p title={data.description}>{data.description}</p>
        </div>
      </>
    );
  };

  return (
    <>
      <EditCommunity
        show={showCommunity}
        hide={handleHideCommunity}
        refresh={refresh}
        id={id}
        initialTitle={data?.title}
        initialDescription={data?.description}
      />

      <StyledContainer>{renderCommunity()}</StyledContainer>
    </>
  );
};
