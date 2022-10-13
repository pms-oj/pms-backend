-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

ALTER TABLE users
DROP COLUMN pk;
ALTER TABLE users
ADD COLUMN pk uuid DEFAULT uuid_generate_v4();
ALTER TABLE users
ADD PRIMARY KEY (pk);

CREATE TABLE teams (
    pk uuid DEFAULT uuid_generate_v4(),
    name VARCHAR NOT NULL,
    PRIMARY KEY (pk)
);

CREATE TABLE team_users (
    pk uuid DEFAULT uuid_generate_v4(),
    user_pk uuid NOT NULL,
    team_pk uuid NOT NULL,
    PRIMARY KEY (pk)
);

CREATE TABLE tasks (
    pk uuid DEFAULT uuid_generate_v4(),
    internal_task_uuid uuid NOT NULL,
    name VARCHAR NOT NULL,
    code VARCHAR NOT NULL,
    is_public BOOLEAN DEFAULT FALSE,
    PRIMARY KEY (pk)
);

CREATE TABLE contest_tasks (
    pk uuid DEFAULT uuid_generate_v4(),
    task_pk uuid NOT NULL,
    contest_pk uuid NOT NULL,
    PRIMARY KEY (pk)
);

CREATE TABLE contests (
    pk uuid DEFAULT uuid_generate_v4(),
    name VARCHAR NOT NULL,
    start_at TIMESTAMPTZ NOT NULL,
    end_at TIMESTAMPTZ NOT NULL,
    is_public BOOLEAN DEFAULT FALSE,
    PRIMARY KEY (pk)
);

CREATE TABLE contest_accessible_users (
    pk uuid DEFAULT uuid_generate_v4(),
    user_pk uuid NOT NULL,
    contest_pk uuid NOT NULL,
    PRIMARY KEY (pk)
);

CREATE TABLE contest_accessible_teams (
    pk uuid DEFAULT uuid_generate_v4(),
    team_pk uuid NOT NULL,
    contest_pk uuid NOT NULL,
    PRIMARY KEY (pk)
);

CREATE TABLE submissions (
    pk uuid DEFAULT uuid_generate_v4(),
    user_pk uuid NOT NULL,
    task_pk uuid NOT NULL,
    lang_uuid uuid NOT NULL,
    issued_at TIMESTAMPTZ DEFAULT NOW(),
    PRIMARY KEY (pk)
);