import React, { useState, useEffect } from "react";
import styled from "styled-components";
import { useAsync } from "react-async";
import { useHistory } from "react-router-dom";

import { CreateCommunity } from "./CreateCommunity";
import { fetchData, Spinner, Error, colors, fonts } from "../helpers";
import { CTAButton } from "../components/CTAButton";
import { ScrollContainer } from "../components/ScrollContainer";

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

const fetchCommunities = async ({ host }) => {
  const hostParam = host ? `?host=${host}` : "";
  return await fetchData(`${process.env.REACT_APP_API}/communities${hostParam}`);
};

export const ListCommunities = ({ host, community, setCommunity }) => {
  const [showCreate, setShowCreate] = useState(false);
  const history = useHistory();

  const handleShowCreate = () => setShowCreate(true);
  const handleHideCreate = () => setShowCreate(false);

  const { data: communities, isLoading, error, reload } = useAsync(fetchCommunities, { host });

  useEffect(() => {
    if (communities && !community) {
      const first = communities[0];
      setCommunity(first);
    }
  }, [communities, community, setCommunity]);

  const handleSelect = (c) => {
    setCommunity(c);
    history.push("/");
  };

  if (isLoading) return <Spinner />;
  if (error) return <Error message={error} />;

  return (
    <StyledCommunities>
      <h1>Communities</h1>
      <CreateCommunity show={showCreate} hide={handleHideCreate} refresh={reload} />
      <ScrollContainer className="communities-list">
        {communities.map((c, i) => (
          <h3 key={i} onClick={() => handleSelect(c)} className={c === community ? "active" : ""}>
            {c}
          </h3>
        ))}
      </ScrollContainer>
      <CTAButton onClick={handleShowCreate}>Create your own!</CTAButton>
    </StyledCommunities>
  );
};
