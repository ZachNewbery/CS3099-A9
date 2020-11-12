import React, { useState, useEffect } from "react";
import styled from "styled-components";
import { fetchData } from "../helpers";
import { useDebouncedCallback } from "use-debounce";
import { Post } from "./SinglePost";

const StyledContainer = styled.div`
  margin: auto;
  width: 500px;
  padding: 1.5em 0;
  display: flex;
  flex-flow: column nowrap;
  align-items: center;
`;

const StyledSearchbar = styled.input`
  border: 1px solid lightgray;
  border-radius: 0.3rem;
  box-sizing: border-box;
  width: 100%;
  outline: none;
  margin: 1em 0;
  padding: 0.75em;
  color: inherit;
  font: inherit;
  font-size: 1em;
`;

export const ListPosts = () => {
  const [search, setSearch] = useState("");
  const [data, setData] = useState(null);

  useEffect(() => {
    const loadPosts = async ({ search }) => {
      const _data = await fetchData(`${process.env.REACT_APP_API_URL}/posts?title_like=${search}`);
      setData(_data)
    }
    loadPosts({ search });
  }, [search]);

  const debouncedSearch = useDebouncedCallback((s) => {
    setSearch(s);
  }, 500);
  


  const renderReponse = () => {
    if (!data) {
      return <h1>Loading</h1>
    }
    if (!data.length) {
      return <p>There are no posts that match this search</p>
    }
    return data.map(post => <Post key={post.id} {...post} />)
  }
  
  return (
    <StyledContainer>
      <StyledSearchbar placeholder="Search posts" onChange={e => debouncedSearch.callback(e.target.value)} />
      {renderReponse()}
    </StyledContainer>
  )
}