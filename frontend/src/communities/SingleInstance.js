import React from "react";
import styled from "styled-components";
import { useAsync } from "react-async";

import { ListCommunities } from "./ListCommunities";


const StyledInstance = styled.div``;

export const SingleInstance = ({ host }) => {


  const renderCommunities = () => {

    return <ListCommunities communities={communities} />;
  };

  return (
    <StyledInstance>
      <h1>{hostLabel}</h1>
      {renderCommunities()}
    </StyledInstance>
  );
};
