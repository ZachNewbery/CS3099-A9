import React, { useContext } from "react";
import styled from "styled-components";
import moment from "moment";
import { useHistory } from "react-router-dom";

import { InstanceContext } from "../App";
import { StyledBlock, StyledContent, renderContent } from "./PostContent";
import { useAsync } from "react-async";
import { fetchData, Spinner, Error } from "../helpers";

import { Post } from "./SinglePost";

const StyledPosts = styled.div`
  padding-bottom: 5rem;
`;

export const ListPosts = ({ posts, isLoading, error }) => {
  const history = useHistory();

  if (isLoading) return <Spinner />;
  if (error) return <Error message={error} />;

  const filteredPosts = posts.filter((post) => !post.deleted);

  return (
    <StyledPosts>
      {filteredPosts.length > 0 ? (
        filteredPosts
          .sort((a, b) => b.created - a.created)
          .map(({ id, title, content, user, created, children }) => (
            <StyledContent key={id} onClick={() => history.push(`/post/${id}`)}>
              <div className="header">
                <h1 className="title" title={title}>
                  {title}
                </h1>
                <div className="date-time">
                  <p className="time">{moment.unix(created).format("HH:mm")}</p>
                  <p className="date">{moment.unix(created).format("DD MMMM YYYY")}</p>
                </div>
              </div>
              {content.map((block, i) => (
                <StyledBlock key={i}>{renderContent(block)}</StyledBlock>
              ))}
            </StyledContent>
          ))
      ) : (
        <p style={{ textAlign: "center" }}>No posts in this community yet</p>
      )}
    </StyledPosts>
  );
};
