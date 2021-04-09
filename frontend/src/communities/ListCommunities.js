import React, { useState, useContext } from "react";
import styled from "styled-components";
import { useHistory } from "react-router-dom";

import { CreateCommunity } from "./CreateCommunity";
import { colors, fonts } from "../helpers";
import { CTAButton } from "../components/CTAButton";
import { ScrollContainer } from "../components/ScrollContainer";

import { CommunityContext } from "../Home";

const StyledCommunities = styled.div`
  display: flex;
  height: 100%;
  flex-flow: column nowrap;
  background: white;
  border: 1px solid ${colors.mediumLightGray};
  border-radius: 0.6rem;
  padding: 1rem 0;

  & > .communities-list {
    flex: 1;
    padding: 0.25rem 1rem;
    margin: 0.25rem 0 1rem;
    & > h3 {
      margin: 0.25rem 0;
      cursor: pointer;
      overflow: hidden;
      white-space: nowrap;
      text-overflow: ellipsis;
      &.active {
        color: ${colors.blue};
      }
    }
  }
  & > h1 {
    margin: 0 1rem;
    font-family: ${fonts.accent};
    font-weight: normal;
    font-size: 1.25rem;
    letter-spacing: 0.5px;
    border-bottom: 1px solid ${colors.veryLightGray};
  }
  & > button {
    margin: 0 1rem;
    width: unset;
  }
`;



export const ListCommunities = ({ communities, refresh }) => {
  const [showCreate, setShowCreate] = useState(false);
  const { community, setCommunity } = useContext(CommunityContext);
  
  const history = useHistory();

  const handleShowCreate = () => setShowCreate(true);
  const handleHideCreate = () => setShowCreate(false);

  const handleSelect = (c) => {
    setCommunity(c);
    history.push("/");
  };

  return (
    <StyledCommunities>
      <h1>Communities</h1>
      <CreateCommunity show={showCreate} hide={handleHideCreate} refresh={refresh} />
      <ScrollContainer className="communities-list">
        {communities.map((c, i) => (
          <h3 key={i} onClick={() => handleSelect(c.id)} className={c.id === community ? "active" : ""} title={c.id}>
            {c.id}
          </h3>
        ))}
      </ScrollContainer>
      <CTAButton onClick={handleShowCreate}>Create your own!</CTAButton>
    </StyledCommunities>
  );
};
