-- Your SQL goes here
CREATE TABLE IF NOT EXISTS Users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(24) NOT NULL
                                 );
CREATE TABLE IF NOT EXISTS Communities (
    id SERIAL PRIMARY KEY,
    uuid VARCHAR(100) NOT NULL,
    descr VARCHAR(255) NOT NULL,
    title VARCHAR(100) NOT NULL
                                       );
CREATE TABLE IF NOT EXISTS Posts (
    id SERIAL PRIMARY KEY,
    uuid VARCHAR(100) NOT NULL,
    title VARCHAR(100) NOT NULL,
    author BIGINT NOT NULL,
    contType VARCHAR(10) NOT NULL,
    body VARCHAR(255) NOT NULL,
    CONSTRAINT FK_author FOREIGN KEY (author) REFERENCES Users(id),
    created DATE NOT NULL, modified DATE
                                 );