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
    CONSTRAINT FK_local_user FOREIGN KEY (userId) REFERENCES Users(id)
);

CREATE TABLE IF NOT EXISTS FederatedUsers (
    id SERIAL PRIMARY KEY,
    userId BIGINT UNSIGNED NOT NULL UNIQUE,
    host TEXT NOT NULL,
    CONSTRAINT FK_federated_user FOREIGN KEY (userId) REFERENCES Users(id)
);

CREATE TABLE IF NOT EXISTS Communities (
    id SERIAL PRIMARY KEY,
    uuid TEXT NOT NULL,
    description TEXT NOT NULL,
    title TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS Posts (
    id SERIAL PRIMARY KEY,
    uuid TEXT NOT NULL,
    title TEXT NOT NULL,
    author BIGINT UNSIGNED NOT NULL,
    contentType BIGINT UNSIGNED NOT NULL,
    body TEXT NOT NULL,
    CONSTRAINT FK_author FOREIGN KEY (author) REFERENCES Users(id),
    created TIMESTAMP NOT NULL,
    modified TIMESTAMP NOT NULL
);