import React, { useEffect, useState } from "react";
import styled from "styled-components";
import moment from "moment";
import { useParams, useHistory } from "react-router-dom";
import { useAsync } from "react-async";

import { fetchData, Spinner, colors, fonts } from "../helpers";
import { StyledBlock, StyledContent, renderContent } from "./PostContent";
import { useUser } from "../index";
import { Comments, CreateComment } from "./Comments";
import { EditPost } from "./EditPost";
import { Profile } from "../components/Profile";

import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faArrowLeft, faPencilAlt, faTrash } from "@fortawesome/free-solid-svg-icons";

const loadSinglePost = async ({ id }) => {
  const post = await fetchData(`${process.env.REACT_APP_API}/posts/${id}`);
  const user = await fetchData(`${process.env.REACT_APP_API}/user/${post.author.id}`);
  post.user = user;
  return post;
};

const deletePost = async ({ id }) => {
  return fetchData(`${process.env.REACT_APP_API}/posts/${id}`, null, "DELETE");
};

const StyledPostContainer = styled.div`
  margin-bottom: 5rem;
  width: 100%;
  background: white;
  border: 1px solid ${colors.mediumLightGray};
  border-radius: 0.6rem;
  display: flex;
  flex-flow: column nowrap;
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
  const [commentCount, setCommentCount] = useState(0);
  const { postId } = useParams();
  const history = useHistory();

  const { data, isLoading, reload } = useAsync(loadSinglePost, { id: postId });

  useEffect(() => {
    if (data && data.community.id !== community) {
      setCommunity(data.community.id);
    }
  }, [data, community, setCommunity]);

  const addComment = () => setCommentCount((c) => c + 1);
  const removeComment = () => setCommentCount((c) => c - 1);

  if (isLoading) {
    return <Spinner />;
  }

  return (
    <StyledPostContainer>
      <Post {...data} refresh={reload} commentCount={commentCount} />
      <CreateComment postId={data.id} refresh={reload} communityId={data.community.id} />
      <Comments children={data.children} addComment={addComment} removeComment={removeComment} />
    </StyledPostContainer>
  );
};

export const Post = ({ id, title, content, created, modified, author, user: _user, refresh, commentCount }) => {
  const history = useHistory();
  const [showEdit, setShowEdit] = useState(false);

  const user = useUser();
  const isAdmin = author.id.toLowerCase() === user.username.toLowerCase() && author.host.toLowerCase() === user.host.toLowerCase();

  const handleEdit = () => {
    setShowEdit(true);
  };

  const handleDelete = async () => {
    await deletePost({ id }).then(history.goBack());
  };

  const isEdited = moment(created).unix() !== moment(modified).unix();

  return (
    <StyledPost>
      <EditPost id={id} hide={() => setShowEdit(false)} show={showEdit} initialTitle={title} initialContent={content} refresh={refresh} />
      <div className="header">
        <div className="back-icon" onClick={() => history.goBack()}>
          <FontAwesomeIcon icon={faArrowLeft} />
        </div>
        <div className="title">
          <h3>{title}</h3>
          <p title={moment(created).format("HH:mma - MMMM D, YYYY")}>{`${moment(created).format("MMMM D, YYYY")} ${isEdited ? "(Edited)" : ""}`}</p>
        </div>
        <div className="profile">
          <div className="details">
            <h3>{author.id}</h3>
            <p title={author.host}>{author.host}</p>
          </div>
          <Profile user={_user} />
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
        <p>{`${commentCount} ${commentCount === 1 ? "comment" : "comments"}`}</p>
      </div>
    </StyledPost>
  );
};
