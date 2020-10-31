-- Your SQL goes here
CREATE TABLE Users ( id SERIAL, username VARCHAR(24), primary key(id));
CREATE TABLE Communities ( id SERIAL, uid VARCHAR(255) NOT NULL, title VARCHAR(255) NOT NULL, primary key(id));
CREATE TABLE CommunitiesAdmins ( communitiesID SERIAL, userID SERIAL, FOREIGN KEY (communitiesID) REFERENCES Communities(id), FOREIGN KEY (userID) REFERENCES Users(id));
CREATE TABLE Posts ( id SERIAL, uuid VARCHAR NOT NULL, title VARCHAR NOT NULL, primary key(id));