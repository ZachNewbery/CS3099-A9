import React, { useContext } from "react";
import styled from "styled-components";
import { useAsync } from "react-async";
import { useHistory } from "react-router-dom";

import { Spinner, Error, colors, fonts, fetchData } from "../helpers";
import { ScrollContainer } from "../components/ScrollContainer";
import { InstanceContext, CommunityContext, SearchContext } from "../App";

import { users } from "../posts/Posts";

const loadPosts = async ({ instance, search }) => {
  const url = new URL(`${process.env.REACT_APP_API}/posts-search`);
  url.searchParams.append("host", instance);
  url.searchParams.append("search", search);
  
  let posts = await fetchData(url);
  
  const editedPosts = posts.map(async (post) => {
    try {
      if (!users[instance]) {
        users[instance] = {};
      }

      if (users[instance][post.author.id]) {
        post.user = users[instance][post.author.id];
      } else {
        try {
          post.user = await fetchData(`${process.env.REACT_APP_API}/user/${post.author.id}?host=${instance}`);
          users[instance][post.author.id] = post.user;
        } catch (e) {}
      }
    } catch (error) {}
    return post;
  });

  posts = await Promise.all([...editedPosts]);
  
  return posts;
};

const StyledPosts = styled.div`
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

const StyledPost = styled.div`
  cursor: pointer;
  padding: 0.5rem 0.75rem;
  box-shadow: inset 0px 0px 0px 1px rgb(255 255 255 / 20%), inset 0 0 10px 2px rgb(255 255 255 / 15%);
  border-radius: 0.5rem;
  margin-top: 0.75rem;
  transition: all 0.2s;
  box-shadow: ${(props) => props.active && "inset 0px 0px 0px 1px rgb(255 255 255 / 30%), inset 0 0 10px 2px rgb(255 255 255 / 42%)"};
  background: ${(props) => props.active && "rgba(255, 255, 255, 0.1)"};
  display: flex;
  align-items: center;
  &:hover {
    box-shadow: inset 0px 0px 0px 1px rgb(255 255 255 / 30%), inset 0 0 10px 2px rgb(255 255 255 / 42%);
  }

  & > h3 {
    margin: 0;
    color: ${colors.white};
    font-size: 0.9rem;
    flex: 1;
  }

  & > p {
    margin: 0;
    color: ${colors.softWhite};
    font-size: 0.825rem;
    padding: 0 0.5rem;
    border-left: 1px solid ${colors.verySoftWhite};
    min-width: 8rem;
    max-width: 8rem;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    text-align: right;
  }
`;

export const ListPosts = ({ search, setIsOpen }) => {
  const history = useHistory();

  const { setSearch } = useContext(SearchContext);
  const { setCommunity } = useContext(CommunityContext);
  const { instance, setInstance, INTERNAL_INSTANCE } = useContext(InstanceContext);

  const { data: posts, isLoading, error } = useAsync(loadPosts, { search, instance });

  const handleClick = (post) => {
    setCommunity(post.community.id);
    setSearch("");
    setIsOpen(false);
    history.push(`/post/${post.id}`);
  };

  if (isLoading) return <Spinner />;
  if (error) return <Error message={error} />;

  const filteredPosts = posts.filter((post) => !post.deleted && post.user);

  return (
    <StyledPosts>
      <h1>{`${filteredPosts.length} result${filteredPosts.length === 1 ? "" : "s"}`}</h1>
      <ScrollContainer
        style={{ maxHeight: "16.7rem", margin: "0 -1.2rem", padding: "0 1.2rem" }}
        scrollcolor="rgba(255, 255, 255, 0.5)"
        scrollhover="rgba(255, 255, 255, 0.7)"
      >
        {filteredPosts.map((post, i) => (
          <StyledPost key={i} onClick={() => handleClick(post)}>
            <h3>{post.title}</h3>
            <p>{post.community.id}</p>
          </StyledPost>
        ))}
      </ScrollContainer>
    </StyledPosts>
  );
};
