import React from "react";
import styled from "styled-components";
import moment from "moment";
import { useHistory } from "react-router-dom";

import { StyledBlock, StyledContent, renderContent } from "./PostContent";
import { useAsync } from "react-async";
import { fetchData, Spinner, Error } from "../helpers";

import { Post } from "./SinglePost";

const loadPosts = async ({ host, community }) => {
  const hostParam = host ? `host=${host}&` : "";
  return fetchData(`${process.env.REACT_APP_API}/posts?${hostParam}community=${community}`);
};

const StyledPosts = styled.div`
  padding-bottom: 5rem;
`;

export const ListPosts = ({ host, community }) => {
  const history = useHistory();

  const { data: posts, isLoading, error } = useAsync(loadPosts, { host, community });

  if (isLoading) return <Spinner />;
  if (error) return <Error message={error} />;

  return (
    <StyledPosts>
      {posts
        .filter(post => !post.deleted)
        .sort((a, b) => moment(b.created).unix() - moment(a.created).unix())
        .map(({ id, title, content, user, created, children }) => (
          <StyledContent key={id} onClick={() => history.push(`/post/${id}`)}>
            <div className="header">
              <h1 className="title" title={title}>
                {title}
              </h1>
              <div className="date-time">
                <p className="time">{moment(created).format("HH:mm")}</p>
                <p className="date">{moment(created).format("DD MMMM YYYY")}</p>
              </div>
            </div>
            {content.map((block, i) => (
              <StyledBlock key={i}>{renderContent(block)}</StyledBlock>
            ))}
          </StyledContent>
        ))}
    </StyledPosts>
  );
};
