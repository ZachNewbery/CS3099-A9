import React from "react";
import styled from "styled-components";
import { useAsync } from "react-async";

import { fetchData, Spinner, Error, colors, fonts } from "../helpers";
import { ListCommunities } from "./ListCommunities";
import { ScrollContainer } from "../components/ScrollContainer";

const fetchInstances = async () => {
  return ["local", "test-instance.com", "another-place.com", "test-instance.com", "another-place.com", "test-instance.com", "another-place.com", "test-instance.com", "another-place.com"];
  // return await fetchData(`${process.env.REACT_APP_API}/fed/servers`);
};

const StyledInstances = styled.div`
  border-top: 1px solid rgba(255, 255, 255, 0.3);
  padding: 1rem 0.5rem;

  & > h1 {
    color: ${colors.white};
    font-family: ${fonts.accent};
    letter-spacing: 0.5px;
    font-weight: normal;
    font-size: 1rem;
    margin: 0;
  }

  .instance {
    cursor: pointer;
    padding: 0.5rem 0.75rem;
    box-shadow: inset 0px 0px 0px 1px rgb(255 255 255 / 20%), inset 0 0 10px 2px rgb(255 255 255 / 15%);
    border-radius: 0.5rem;
    margin-top: 0.75rem;
    transition: all 0.2s;
    &:hover {
      box-shadow: inset 0px 0px 0px 1px rgb(255 255 255 / 30%), inset 0 0 10px 2px rgb(255 255 255 / 42%);
    }
    
    & > h3 {
      margin: 0;
      color: ${colors.white};
      font-size: 0.9rem;
    }
  }
`;

export const ListInstances = () => {
  const { data: instances, isLoading, error } = useAsync(fetchInstances);

  if (isLoading) return <Spinner />;
  if (error) return <Error message={error} />;

  return (
    <StyledInstances>
      <h1>Instances</h1>
      <ScrollContainer style={{ maxHeight: "16.7rem", margin: "0.5rem -1.2rem", padding: "0 1.2rem" }} scrollcolor="rgba(255, 255, 255, 0.5)" scrollhover="rgba(255, 255, 255, 0.7)">
        {instances.map((instance, i) => (
          <div className="instance" key={i}>
            <h3>{instance}</h3>
          </div>
        ))}
      </ScrollContainer>
    </StyledInstances>
  );
};
