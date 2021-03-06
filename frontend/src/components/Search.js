import React, { useState, useContext, useEffect, useRef } from "react";
import styled from "styled-components";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faSearch } from "@fortawesome/free-solid-svg-icons";
import { colors } from "../helpers";
import { InstanceContext, SearchContext } from "../App";
import { ListInstances } from "../communities/ListInstances";
import { ListPosts } from "../communities/ListPosts";
import { useDebouncedCallback } from "use-debounce";

const StyledSearch = styled.div`
  width: 35rem;
  height: 2.5rem;
  border-radius: 1.25rem;
  background: ${colors.blueGradient};
  overflow: hidden;
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
      margin-left: 1rem;
    }

    & > .current-instance {
      cursor: pointer;
      box-shadow: ${colors.blueInsetShadow};
      margin: 0;
      color: ${colors.white};
      font-size: 0.85rem;
      height: calc(100% - 0.5rem);
      display: flex;
      justify-content: center;
      align-items: center;
      border-radius: 1.25rem;
      padding: 0 1rem;
      margin: 0.5rem;
      box-sizing: border-box;
      & > p {
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
        max-width: 12rem;
      }

      transition: all 0.1s ease-out;
      &:hover {
        box-shadow: ${colors.blueInsetShadow}, inset 0px 0px 0px 1px rgb(255 255 255 / 30%), inset 0 0 10px 2px rgb(255 255 255 / 42%);
      }
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
  }
  & > .instance-area {
    padding: 1rem;
  }
`;

export const Search = () => {
  const { search, setSearch } = useContext(SearchContext);
  const searchRef = useRef(null);

  const [isOpen, setIsOpen] = useState(false);
  const { instance, setInstance } = useContext(InstanceContext);

  const handleClose = () => {
    setIsOpen(false);
    searchRef.current.value = "";
  };

  const [handleChange] = useDebouncedCallback((e) => {
    const text = e.target.value;
    setSearch(text);
  }, 300);

  useEffect(() => {
    handleClose();
  }, [instance]);

  return (
    <>
      <StyledSearch className={isOpen ? "open" : ""}>
        <div className="search-area">
          <FontAwesomeIcon icon={faSearch} />
          <input
            className="search-control"
            onChange={handleChange}
            defaultValue={search}
            placeholder="Search"
            ref={searchRef}
            onClick={() => setIsOpen("posts")}
          />
          <div className={`current-instance ${isOpen === "instance" ? "active" : ""}`} onClick={() => setIsOpen("instance")}>
            <p title={instance || "All hosts"}>{instance || "All hosts"}</p>
          </div>
        </div>
        <div className="instance-area">
          {isOpen ? (
            isOpen === "instance" ? (
              <ListInstances instance={instance} setInstance={setInstance} />
            ) : (
              <ListPosts instance={instance} search={search} key={search} setIsOpen={setIsOpen} />
            )
          ) : null}
        </div>
      </StyledSearch>
      {isOpen && <StyledBackground onClick={handleClose} />}
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
