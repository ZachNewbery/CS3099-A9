import React, { useState } from "react";
import styled from "styled-components";
import { useAsync } from "react-async";

import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faPencilAlt, faTrash } from "@fortawesome/free-solid-svg-icons";
import { fetchData, Spinner, Error, colors, fonts } from "../helpers";

import { EditCommunity } from "./EditCommunity";

const fetchCommunities = async ({ id, host }) => {
  const hostParam = host ? `?host=${host}` : "";
  return await fetchData(`${process.env.REACT_APP_API}/communities/${id}${hostParam}`);
};

const StyledContainer = styled.div`
  display: flex;
  flex-flow: column nowrap;
  background: ${colors.blueGradient};
  box-shadow: ${colors.blueInsetShadow};
  border-radius: 0.6rem;
  margin-bottom: 1rem;
  padding: 1rem 0;
  position: relative;

  & > h1 {
    margin: 0 1rem;
    font-family: ${fonts.accent};
    color: ${colors.white};
    font-weight: normal;
    font-size: 1.25rem;
    letter-spacing: 0.5px;
  }

  & > .actions {
    position: absolute;
    top: 0.75rem;
    right: 0.75rem;
    display: flex;
    & > svg {
      color: ${colors.softWhite};
      transition: all 0.3s;
      font-size: 1rem;
      margin-left: 0.75rem;
      cursor: pointer;
      &:hover {
        color: ${colors.white};
      }
    }
  }

  & > .content {
    & > p {
      color: ${colors.white};
      margin: 1rem;
      font-size: 1rem;
    }
  }
`;

const deleteCommunity = async ({ id }) => {
  return await fetchData(`${process.env.REACT_APP_API}/communities`, JSON.stringify({ id }), "DELETE");
};

export const SingleCommunity = ({ id, host }) => {
  const [showCommunity, setShowCommunity] = useState(false);

  const { data, isLoading, error, reload } = useAsync(fetchCommunities, { id, host });

  const handleShowCommunity = () => setShowCommunity(true);
  const handleHideCommunity = () => setShowCommunity(false);

  const handleEdit = () => handleShowCommunity();
  const handleDelete = async e => {
    e.preventDefault();
    await deleteCommunity({ id });
  }
  
  const renderCommunity = () => {
    if (isLoading) return <Spinner />;
    if (error) return <Error message={error} />;

    return (
      <div className="content">
        <p>{data.description}</p>
      </div>
    );
  };

  return (
    <>
      <EditCommunity
        show={showCommunity}
        hide={handleHideCommunity}
        refresh={reload}
        id={id}
        initialTitle={data?.title}
        initialDescription={data?.description}
      />

      <StyledContainer>
        <h1>{id}</h1>
        {renderCommunity()}
        <div className="actions">
          <FontAwesomeIcon onClick={handleEdit} icon={faPencilAlt} />
          <FontAwesomeIcon onClick={handleDelete} icon={faTrash} />
        </div>
      </StyledContainer>
    </>
  );
};
