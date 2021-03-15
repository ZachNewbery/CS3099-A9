import React, { useEffect, useState } from "react";
import styled from "styled-components";
import moment from "moment";
import { useParams, useHistory } from "react-router-dom";
import { useAsync } from "react-async";

import { fetchData, Spinner, colors, fonts } from "../helpers";
import { StyledBlock, StyledContent, renderContent } from "./PostContent";
import { Comments, CreateComment } from "./Comments";
import { EditPost } from "./EditPost";

import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faArrowLeft, faPencilAlt, faTrash } from "@fortawesome/free-solid-svg-icons";

const loadSinglePost = async ({ id }) => {
  return fetchData(`${process.env.REACT_APP_API}/posts/${id}`);
};

const deletePost = async ({ id }) => {
  return fetchData(`${process.env.REACT_APP_API}/posts`, JSON.stringify({ id }), "DELETE");
}

const StyledPostContainer = styled.div`
  width: 100%;
  background: white;
  border: 1px solid ${colors.mediumLightGray};
  border-radius: 0.6rem;
  display: flex;
  flex-flow: column nowrap;
  overflow: hidden;
`;

const StyledPost = styled.div`
  & > .header {
    display: flex;
    align-items: center;
    border-bottom: 1px solid ${colors.veryLightGray};
    padding: 0.6rem 0.6rem;

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
      & > img {
        height: 2.5rem;
        width: 2.5rem;
        border-radius: 1.5rem;
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

  & > .content {
    padding: 1rem;
  }

  & > .footer {
    border-top: 1px solid ${colors.veryLightGray};
    border-bottom: 1px solid ${colors.veryLightGray};
    padding: 0.6rem 0.2rem;
    margin: 0 0.8rem;
    display: flex;
    align-items: center;
    & > .actions {
      flex: 1;
      display: flex;
      align-items: center;
      & > svg {
        margin-right: 0.5rem;
        color: ${colors.darkGray};
        cursor: pointer;
        transition: all 0.3s;
        &:hover {
          color: ${colors.gray};
        }
      }
    }
    & > p {
      margin: 0;
      font-size: 0.8rem;
      color: ${colors.lightText};
    }
  }
`;

export const SinglePost = ({ community, setCommunity }) => {
  const { postId } = useParams();
  const history = useHistory();

  const { data, isLoading, reload } = useAsync(loadSinglePost, { id: postId });

  useEffect(() => {
    if (data && data.community.id !== community) {
      setCommunity(data.community.id);
    }
  }, [data, community, setCommunity]);

  if (isLoading) {
    return <Spinner />;
  }

  return (
    <StyledPostContainer>
      <Post {...data} refresh={reload} />
      <CreateComment postId={data.id} refresh={reload} communityId={data.community.id} />
      <Comments children={data.children} />
    </StyledPostContainer>
  );
};

export const Post = ({ id, title, content, created, children, author, refresh }) => {
  const history = useHistory();
  const [showEdit, setShowEdit] = useState(false);

  const currentUser = { id: "", host: "" };
  const isAdmin = true || (author.id.toLowerCase() === currentUser.id.toLowerCase() && author.host.toLowerCase() === currentUser.host.toLowerCase());

  const handleEdit = () => {
    setShowEdit(true);
  }

  const handleDelete = async () => {
    await deletePost({ id });
  }
  
  return (
    <StyledPost>
      <EditPost id={id} hide={() => setShowEdit(false)} show={showEdit} initialTitle={title} initialContent={content[0].text} refresh={refresh} />
      <div className="header">
        <div className="back-icon" onClick={() => history.goBack()}>
          <FontAwesomeIcon icon={faArrowLeft} />
        </div>
        <div className="title">
          <h3>{title}</h3>
          <p title={moment(created).format("HH:mma - MMMM D, YYYY")}>{moment(created).format("MMMM D, YYYY")}</p>
        </div>
        <div className="profile">
          <div className="details">
            <h3>{author.id}</h3>
            <p title={author.host}>{author.host}</p>
          </div>
          <img
            alt="profile"
            src={`https://eu.ui-avatars.com/api/?rounded=true&bold=true&background=0061ff&color=ffffff&uppercase=true&format=svg&name=${author.id}`}
          />
        </div>
      </div>
      <div className="content">
        {content.map((block, i) => (
          <StyledBlock key={i}>{renderContent(block)}</StyledBlock>
        ))}
      </div>
      <div className="footer">
        {isAdmin && (
          <div className="actions">
            <FontAwesomeIcon onClick={handleEdit} icon={faPencilAlt} />
            <FontAwesomeIcon onClick={handleDelete} icon={faTrash} />
          </div>
        )}
        <p>{`${children.length} ${children.length === 1 ? "comment" : "comments"}`}</p>
      </div>
    </StyledPost>
  );
};
