import React, { useState, useEffect } from "react";
import styled from "styled-components";
import { useAsync } from "react-async";
import { fetchData } from "../helpers";
import { useDebouncedCallback } from "use-debounce";
import { Post } from "./SinglePost";

const loadCommunities = async () => {
  return fetchData(`${process.env.REACT_APP_API_URL}/communities`);
};

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

const StyledCommunityPicker = styled.select`
  border: 1px solid lightgray;
  border-radius: 0.3em;
  padding: 0.5em;
  margin: 0.5em 0;
  width: 100%;
`;

const ALL_COMMUNITY = {
  title: "all communities",
  id: "-1",
  userIds: []
}

export const ListPosts = () => {
  const [search, setSearch] = useState("");
  const [posts, setPosts] = useState(null);
  const [community, setCommunity] = useState(null);

  const { data: communities } = useAsync(loadCommunities);

  useEffect(() => {
    const loadPosts = async ({ search, community }) => {
      const communityParam =
        community !== ALL_COMMUNITY.id ? `&communityId=${community}` : "";
      const _data = await fetchData(
        `${process.env.REACT_APP_API_URL}/posts?title_like=${search}${communityParam}`
      );
      setPosts(_data);
    };
    if (community) loadPosts({ search, community });
  }, [search, community]);

  useEffect(() => {
    if (communities) {
      setCommunity(communities[0].id);
    }
  }, [communities]);

  const debouncedSearch = useDebouncedCallback((s) => {
    setSearch(s);
  }, 500);

  const renderReponse = () => {
    if (!posts) {
      return <h1>Loading</h1>;
    }
    if (!posts.length) {
      return <p>There are no posts that match this search</p>;
    }
    return posts.map((post) => <Post key={post.id} {...post} />);
  };

  const handleSelectCommunity = (e) => {
    const id = e.target.selectedOptions[0].value;
    setCommunity(id);
  };

  const currentCommunity = communities && (communities?.find((c) => c.id === community) || ALL_COMMUNITY);

  return (
    <StyledContainer>
      <StyledCommunityPicker onChange={handleSelectCommunity}>
        {communities && (
          <>
            {communities.map(({ id, title }) => (
              <option key={id} value={id}>
                {title}
              </option>
            ))}
            <option value={ALL_COMMUNITY.id}>{ALL_COMMUNITY.title}</option>
          </>
        )}
      </StyledCommunityPicker>
      <StyledSearchbar
        placeholder={
          currentCommunity
            ? `Search posts in ${currentCommunity.title}`
            : "Loading communities"
        }
        onChange={(e) => debouncedSearch.callback(e.target.value)}
      />
      {renderReponse()}
    </StyledContainer>
  );
};
