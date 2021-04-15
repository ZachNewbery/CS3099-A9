import React, { useRef, useState, useContext } from "react";
import styled from "styled-components";
import moment from "moment";
import { useAsync } from "react-async";

import { InstanceContext, CommunityContext } from "../App";

import { fetchData, Spinner, Error, colors } from "../helpers";
import { StyledBlock, StyledContent, renderContent } from "./PostContent";
import { useUser } from "../index";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faPencilAlt, faTrash } from "@fortawesome/free-solid-svg-icons";
import { EditComment } from "./EditComment";
import { useDebouncedCallback } from "use-debounce";
import { Tooltip } from "../components/Tooltip";
import { Profile } from "../components/Profile";

import { users } from "./Posts";

const loadChildPosts = async ({ children, community, instance, addComment }) => {
  let posts = children.map(async (child) => {
    const url = new URL(`${process.env.REACT_APP_API}/posts/${child}`);
    const appendParam = (key, value) => value && url.searchParams.append(key, value);
    appendParam("host", instance);
    appendParam("community", community);
    const post = await fetchData(url);

    if (!users[instance]) {
      users[instance] = {};
    }

    let user;

    if (users[instance][post.author.id]) {
      user = users[instance][post.author.id];
    } else {
      user = await fetchData(`${process.env.REACT_APP_API}/user/${post.author.id}`);
    }
    if (!post.deleted) addComment();
    post.user = user;
    return post;
  });

  posts = await Promise.all([...posts]);

  return posts;
};

const deletePost = async ({ id, community, instance }) => {
  const url = new URL(`${process.env.REACT_APP_API}/posts/${id}`);
  const appendParam = (key, value) => value && url.searchParams.append(key, value);
  appendParam("host", instance);
  appendParam("community", community);
  return fetchData(url, null, "DELETE");
};

const createComment = async ({ postId, communityId, instance, content }) => {
  const post = {
    parent: postId,
    content: [
      {
        markdown: {
          text: content,
        },
      },
    ],
    community: {
      id: communityId,
    },
    title: null,
  };

  const url = new URL(`${process.env.REACT_APP_API}/posts/create`);
  const appendParam = (key, value) => value && url.searchParams.append(key, value);
  appendParam("host", instance);
  appendParam("community", communityId);
  return fetchData(url, JSON.stringify(post), "POST");
};

const StyledComments = styled.div`
  background: ${colors.pageBackground};
  padding: 1rem 0;
  & > .comment {
    padding: 0 1rem;
    &:not(:first-child) {
      padding-top: 1rem;
    }
    & > .main {
      display: flex;
      align-items: center;
      & > .profile > div {
        margin-right: 0.5rem;
      }
      & > .content {
        width: 100%;
        background: white;
        border: 1px solid ${colors.mediumLightGray};
        border-radius: 0.6rem;
        padding: 0 1rem;
        .mde-preview-content {
          padding: 0;
        }
      }
    }
    & > .footer {
      display: flex;
      margin: 0.25rem;
      margin-left: 3.5rem;
      & > p {
        margin: 0;
        font-size: 0.75rem;
      }
      & > .actions {
        flex: 1;
        display: flex;
        align-items: center;
        margin-left: 0.5rem;
        border-left: 1px solid ${colors.lightGray};
        padding-left: 0.5rem;
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
    }
  }
`;

const StyledCreateComment = styled.div`
  padding: 0.8rem 0;
  margin: 0 0.8rem;
  label {
    position: relative;
  }
  input {
    border: 1px solid ${colors.veryLightGray};
    width: 100%;
    border-radius: 0.6em;
    padding: 0.5rem 0.7rem;
    color: inherit;
    font: inherit;
    font-size: 0.8rem;
    outline: none;
  }
`;

export const CreateComment = ({ postId, communityId, refresh }) => {
  const { instance } = useContext(InstanceContext);

  const contentRef = useRef(null);
  const [errors, setErrors] = useState({});

  const handleSubmit = async (e) => {
    e.preventDefault();
    let currentErrors = {};

    const content = contentRef.current.value;

    if (content.length < 5) {
      currentErrors.content = "Too short";
    }

    if (Object.keys(currentErrors).length === 0) {
      try {
        contentRef.current.value = "";
        await createComment({ postId, content, instance, communityId });
        return refresh();
      } catch (error) {
        currentErrors.content = error.message;
      }
    }

    setErrors(currentErrors);
  };

  const [handleKeyDownDebounced] = useDebouncedCallback(async (e) => {
    if (e.key === "Enter") {
      await handleSubmit(e);
    }
  }, 500);

  return (
    <StyledCreateComment>
      <label>
        <input ref={contentRef} placeholder="Write a comment..." onKeyDown={handleKeyDownDebounced} onChange={() => setErrors({})} />
        {errors.content && <Tooltip text={errors.content} />}
      </label>
    </StyledCreateComment>
  );
};

export const Comments = ({ children, addComment, removeComment, setCommentCount, refreshParent }) => {
  const { instance } = useContext(InstanceContext);
  const { community } = useContext(CommunityContext);

  const { data: comments, isLoading, error, reload } = useAsync(loadChildPosts, { children, instance, community, addComment });
  const [showEdit, setShowEdit] = useState({ showModal: false, content: null, id: null });

  const refresh = () => {
    setCommentCount(0);
    reload();
  };

  const user = useUser();

  if (isLoading) return <Spinner />;
  if (error) return <Error message={error} />;

  return (
    <>
      <EditComment
        id={showEdit.id}
        hide={() => setShowEdit({ showModal: false })}
        show={showEdit.showModal}
        initialContent={showEdit.content}
        refresh={refresh}
        instance={instance}
        community={community}
      />
      <StyledComments>
        {comments
          .filter((post) => !post.deleted)
          .sort((a, b) => moment(b.created).unix() - moment(a.created).unix())
          .map((comment) => {
            const { author } = comment;
            const isAdmin = author.id.toLowerCase() === user.id.toLowerCase() && author.host.toLowerCase() === user.host.toLowerCase();

            const handleEdit = () => {
              setShowEdit({ showModal: true, content: comment.content, id: comment.id });
            };

            const handleDelete = async () => {
              await deletePost({ id: comment.id, instance, community }).then(() => refreshParent());
            };

            const isEdited = comment.created !== comment.modified;

            return (
              <div key={comment.id} className="comment">
                <div className="main">
                  <div className="profile">
                    <Profile user={comment.user} />
                  </div>
                  <div className="content">
                    {comment.content.map((block, i) => (
                      <StyledBlock key={i}>{renderContent(block)}</StyledBlock>
                    ))}
                  </div>
                </div>
                <div className="footer">
                  <p title={moment.unix(comment.created).format("HH:mma - Do MMMM, YYYY")}>{`${moment.unix(comment.created).fromNow()} ${
                    isEdited ? "(Edited)" : ""
                  }`}</p>
                  {isAdmin && (
                    <div className="actions">
                      <FontAwesomeIcon onClick={handleEdit} icon={faPencilAlt} />
                      <FontAwesomeIcon onClick={handleDelete} icon={faTrash} />
                    </div>
                  )}
                </div>
              </div>
            );
          })}
      </StyledComments>
    </>
  );
};
