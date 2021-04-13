import React, { useContext } from "react";
import styled from "styled-components";
import moment from "moment";
import { useHistory } from "react-router-dom";

import { Profile } from "../components/Profile";
import { InstanceContext } from "../App";
import { StyledBlock, StyledContent, renderContent } from "./PostContent";
import { useAsync } from "react-async";
import { fetchData, Spinner, Error, colors } from "../helpers";

import { Post } from "./SinglePost";

const StyledPosts = styled.div`
  padding-bottom: 3rem;

  .header {
    display: flex;
    align-items: center;
    border-bottom: 1px solid ${colors.veryLightGray};
    padding: 1rem;

    & > .back-icon {
      height: 2rem;
      width: 2rem;
      display: flex;
      justify-content: center;
      align-items: center;
      border-radius: 1rem;
      margin-right: 0.75rem;

      cursor: pointer;
      transition: all 0.2s;
      &:hover {
        background: ${colors.veryLightGray};
      }

      & > svg {
        font-size: 1rem;
        color: ${colors.darkGray};
      }
    }

    & > .title {
      flex: 1;
      & > h3 {
        margin: 0;
        color: ${colors.lightText};
        font-size: 1.15rem;
      }
      & > p {
        margin: 0;
        color: ${colors.lightGray};
        font-size: 0.7rem;
        max-width: 10rem;
        overflow: hidden;
        white-space: nowrap;
        text-overflow: ellipsis;
      }
    }

    & > .profile {
      display: flex;
      & > .profile-picture {
        margin-left: 1rem;
      }

      & > .details {
        justify-content: center;
        display: flex;
        flex-flow: column;
        & > h3 {
          margin: 0;
          color: ${colors.lightText};
          font-size: 0.9rem;
          text-align: right;
          line-height: 1;
        }
        & > p {
          margin: 0;
          color: ${colors.lightGray};
          font-size: 0.7rem;
          text-align: right;
          max-width: 10rem;
          overflow: hidden;
          white-space: nowrap;
          text-overflow: ellipsis;
        }
      }
    }
  }
`;

export const ListPosts = ({ posts, isLoading, error }) => {
  const history = useHistory();

  if (isLoading) return <Spinner />;
  if (error) return <Error message={error} />;

  const filteredPosts = posts.filter((post) => !post.deleted && post.user);

  return (
    <StyledPosts>
      {filteredPosts.length > 0 ? (
        filteredPosts
          .sort((a, b) => b.created - a.created)
          .map(({ id, title, content, author, user, created, modified }) => {
            const isEdited = created !== modified;

            return (
              <StyledContent key={id} onClick={() => history.push(`/post/${id}`)}>
                <div className="header">
                  <div className="title">
                    <h3>{title}</h3>
                    <p title={moment.unix(created).format("HH:mma - MMMM D, YYYY")}>{`${moment.unix(created).format("MMMM D, YYYY")} ${
                      isEdited ? "(Edited)" : ""
                    }`}</p>
                  </div>
                  <div className="profile">
                    <div className="details">
                      <h3>{author.id}</h3>
                      <p title={author.host}>{author.host}</p>
                    </div>
                    <Profile user={user} />
                  </div>
                </div>
                {content.map((block, i) => (
                  <StyledBlock key={i}>{renderContent(block)}</StyledBlock>
                ))}
              </StyledContent>
            );
          })
      ) : (
        <p style={{ textAlign: "center" }}>No posts in this community yet</p>
      )}
    </StyledPosts>
  );
};
