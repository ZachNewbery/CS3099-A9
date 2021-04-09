import React, { useState, useContext, useEffect } from "react";
import styled from "styled-components";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faSearch } from "@fortawesome/free-solid-svg-icons";
import { colors } from "../helpers";
import { InstanceContext, SearchContext } from "../App";
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
  &.open {
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
  const { search, setSearch } = useContext(SearchContext)
  
  const [isOpen, setIsOpen] = useState(false);
  const { instance, setInstance } = useContext(InstanceContext);

  const handleChange = (e) => {
    const text = e.target.value;
    setSearch(text);
  };

  useEffect(() => {
    setIsOpen(false);
  }, [instance])

  return (
    <>
      <StyledSearch className={isOpen ? "open" : ""} onClick={() => setIsOpen(true)}>
        <div className="search-area">
          <FontAwesomeIcon icon={faSearch} />
          <input className="search-control" onChange={handleChange} placeholder="Search" />
          <p title={instance || "All hosts"}>{instance || "All hosts"}</p>
        </div>
        <div className="instance-area">
          <ListInstances instance={instance} setInstance={setInstance} />
        </div>
      </StyledSearch>
      {isOpen && <StyledBackground onClick={() => setIsOpen(false)} />}
    </>
  );
};

const StyledBackground = styled.div`
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: transparent;
  z-index: -1;
`;
