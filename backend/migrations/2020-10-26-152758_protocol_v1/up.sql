-- Your SQL goes here
CREATE TABLE IF NOT EXISTS Users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(24) NOT NULL
                                 );

CREATE TABLE IF NOT EXISTS LocalUsers (
    id SERIAL PRIMARY KEY,
    userId BIGINT UNSIGNED NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL DEFAULT 'hunter2', -- TODO: Make this secure
    CONSTRAINT FK_local_user FOREIGN KEY (userId) REFERENCES Users(id)
);

CREATE TABLE IF NOT EXISTS FederatedUsers (
    id SERIAL PRIMARY KEY,
    userId BIGINT UNSIGNED NOT NULL UNIQUE,
    host VARCHAR(255) NOT NULL,
    CONSTRAINT FK_federated_user FOREIGN KEY (userId) REFERENCES Users(id)
);

CREATE TABLE IF NOT EXISTS Communities (
    id SERIAL PRIMARY KEY,
    uuid VARCHAR(100) NOT NULL,
    description VARCHAR(255) NOT NULL,
    title VARCHAR(100) NOT NULL
                                       );
CREATE TABLE IF NOT EXISTS Posts (
    id SERIAL PRIMARY KEY,
    uuid VARCHAR(100) NOT NULL,
    title VARCHAR(100) NOT NULL,
    author BIGINT UNSIGNED NOT NULL UNIQUE,
    contentType BIGINT UNSIGNED NOT NULL,
    body VARCHAR(255) NOT NULL,
    CONSTRAINT FK_author FOREIGN KEY (author) REFERENCES Users(id),
    created DATETIME NOT NULL,
    modified DATETIME NOT NULL
                                 );