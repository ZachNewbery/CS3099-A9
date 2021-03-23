-- Your SQL goes here
CREATE TABLE IF NOT EXISTS Users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(24) NOT NULL
);

CREATE TABLE IF NOT EXISTS LocalUsers (
    id SERIAL PRIMARY KEY,
    userId BIGINT UNSIGNED NOT NULL UNIQUE,
    email VARCHAR(254) NOT NULL UNIQUE,
    password TEXT NOT NULL DEFAULT 'hunter2',   -- TODO: Make this secure
    createdAt TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    session VARCHAR(36) NOT NULL DEFAULT '',    -- JWT
    CONSTRAINT FK_LocalUsers_userId FOREIGN KEY (userId) REFERENCES Users(id),
    bio TEXT,
    avatar TEXT
);

CREATE TABLE IF NOT EXISTS FederatedUsers (
    id SERIAL PRIMARY KEY,
    userId BIGINT UNSIGNED NOT NULL UNIQUE,
    host TEXT NOT NULL,
    CONSTRAINT FK_FederatedUsers_userId FOREIGN KEY (userId) REFERENCES Users(id)
);

CREATE TABLE IF NOT EXISTS Communities (
    id SERIAL PRIMARY KEY,
    name VARCHAR(254) NOT NULL UNIQUE,
    description TEXT NOT NULL,
    title TEXT NOT NULL
);

# Admins
CREATE TABLE IF NOT EXISTS CommunitiesUsers (
    id SERIAL PRIMARY KEY,
    communityId BIGINT UNSIGNED NOT NULL,
    CONSTRAINT FK_CommunitiesUsers_communityId FOREIGN KEY (communityId) REFERENCES Communities(id),
    userId BIGINT UNSIGNED NOT NULL,
    CONSTRAINT FK_CommunitiesUsers_userId FOREIGN KEY (userId) REFERENCES Users(id)
);

CREATE TABLE IF NOT EXISTS Posts (
    id SERIAL PRIMARY KEY,
    uuid VARCHAR(36) NOT NULL UNIQUE,
    title TEXT NOT NULL,
    authorId BIGINT UNSIGNED NOT NULL,
    CONSTRAINT FK_Posts_authorId FOREIGN KEY (authorId) REFERENCES Users(id),
    created TIMESTAMP NOT NULL,
    modified TIMESTAMP NOT NULL,
    parentId BIGINT UNSIGNED,
    CONSTRAINT FK_Posts_parentId FOREIGN KEY (parentId) REFERENCES Posts(id),
    communityId BIGINT UNSIGNED NOT NULL,
    CONSTRAINT FK_Posts_communityId FOREIGN KEY (communityId) REFERENCES Communities(id),
    deleted BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS Text (
    id SERIAL PRIMARY KEY,
    content TEXT NOT NULL,
    postId BIGINT UNSIGNED NOT NULL,
    CONSTRAINT FK_Text_postId FOREIGN KEY (postId) REFERENCES Posts(id)
);

CREATE TABLE IF NOT EXISTS Markdown (
    id SERIAL PRIMARY KEY,
    content TEXT NOT NULL,
    postId BIGINT UNSIGNED NOT NULL,
    CONSTRAINT FK_Markdown_postId FOREIGN KEY (postId) REFERENCES Posts(id)
);