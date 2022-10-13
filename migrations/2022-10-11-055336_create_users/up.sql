-- Your SQL goes here

CREATE TABLE users (
    pk SERIAL PRIMARY KEY,
    id VARCHAR NOT NULL,
    pass VARCHAR NOT NULL,
    permission INT NOT NULL
);