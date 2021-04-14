import React, { useContext } from "react";
import styled from "styled-components";
import { useAsync } from "react-async";
import { useHistory } from "react-router-dom";

import { Spinner, Error, colors, fonts, fetchData } from "../helpers";
import { ScrollContainer } from "../components/ScrollContainer";
import { InstanceContext, CommunityContext } from "../App";

const loadInstances = async () => {
  return await fetchData(`${process.env.REACT_APP_API}/servers`);
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
`;

const StyledInstance = styled.div`
  cursor: pointer;
  padding: 0.5rem 0.75rem;
  box-shadow: inset 0px 0px 0px 1px rgb(255 255 255 / 20%), inset 0 0 10px 2px rgb(255 255 255 / 15%);
  border-radius: 0.5rem;
  margin-top: 0.75rem;
  transition: all 0.2s;
  box-shadow: ${(props) => props.active && "inset 0px 0px 0px 1px rgb(255 255 255 / 30%), inset 0 0 10px 2px rgb(255 255 255 / 42%)"};
  background: ${(props) => props.active && "rgba(255, 255, 255, 0.1)"};
  &:hover {
    box-shadow: inset 0px 0px 0px 1px rgb(255 255 255 / 30%), inset 0 0 10px 2px rgb(255 255 255 / 42%);
  }

  & > h3 {
    margin: 0;
    color: ${colors.white};
    font-size: 0.9rem;
  }
`;

export const ListInstances = () => {
  const { instance, setInstance, INTERNAL_INSTANCE } = useContext(InstanceContext);
  const { setCommunity } = useContext(CommunityContext);
  const history = useHistory();

  const { data: instances, isLoading, error } = useAsync(loadInstances);

  const handleClick = (instance) => {
    history.push("/");
    setInstance(instance);
    setCommunity(null);
  };

  if (isLoading) return <Spinner />;
  if (error) return <Error message={error} />;

  return (
    <StyledInstances>
      <h1>Instances</h1>
      <ScrollContainer
        style={{ maxHeight: "16.7rem", margin: "0 -1.2rem", padding: "0 1.2rem 1.2rem" }}
        scrollcolor="rgba(255, 255, 255, 0.5)"
        scrollhover="rgba(255, 255, 255, 0.7)"
      >
        <StyledInstance active={instance === INTERNAL_INSTANCE} onClick={() => handleClick(INTERNAL_INSTANCE)}>
          <h3>{INTERNAL_INSTANCE}</h3>
        </StyledInstance>
        {instances.map((inst, i) => (
          <StyledInstance key={i} active={instance === inst} onClick={() => handleClick(inst)}>
            <h3>{inst}</h3>
          </StyledInstance>
        ))}
      </ScrollContainer>
    </StyledInstances>
  );
};
