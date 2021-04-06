import React, { useState, useContext } from "react";
import styled from "styled-components";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faSearch } from "@fortawesome/free-solid-svg-icons";
import { colors } from "../helpers";
import { InstanceContext } from "../App";
import { ListInstances } from "../communities/ListInstances";

const StyledSearch = styled.div`
  width: 25rem;
  height: 2.5rem;
  border-radius: 1.25rem;
  background: ${colors.blueGradient};
  overflow: hidden;
  padding: 0 1rem;
  box-shadow: ${colors.blueInsetShadow};

  transition: all 0.3s;
  &:focus,
  &:focus-within {
    height: 23rem;
    box-shadow: ${colors.blueInsetShadow}, 0 10px 25px -10px rgb(9 98 189 / 64%), 0 40px 70px -15px rgb(32 89 234 / 79%);
  }

  & > .search-area {
    display: flex;
    align-items: center;
    height: 2.5rem;

    & > svg {
      color: ${colors.white};
      font-size: 1rem;
    }

    & > .search-control {
      height: 100%;
      width: 100%;
      border: none;
      outline: none;
      padding: 0.75rem;
      background: none;
      font: inherit;
      font-size: 1rem;
      color: ${colors.white};
      flex: 1;
      ::placeholder {
        color: ${colors.softWhite};
      }
    }

    & > p {
      margin: 0;
      color: ${colors.white};
      font-size: 0.85rem;
      max-width: 8rem;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }
  }
`;

export const Search = () => {
  const [search, setSearch] = useState(null);
  const { instance, setInstance } = useContext(InstanceContext);

  const handleChange = (e) => {
    const text = e.target.value;
    setSearch(text);
  };

  return (
    <StyledSearch>
      <div className="search-area">
        <FontAwesomeIcon icon={faSearch} />
        <input className="search-control" onChange={handleChange} placeholder="Search" />
        <p title={instance}>{instance}</p>
      </div>
      <div className="instance-area">
        <ListInstances instance={instance} setInstance={setInstance} />
      </div>
    </StyledSearch>
  );
};
