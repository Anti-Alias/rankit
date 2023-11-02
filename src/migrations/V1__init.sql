CREATE TYPE role AS ENUM ('basic', 'root');

CREATE TABLE account (
    id          serial NOT NULL,
    name        text NOT NULL UNIQUE,
    email       text NOT NULL UNIQUE,
    password    text NOT NULL,
    role        role NOT NULL DEFAULT 'basic',
    deleted     boolean NOT NULL DEFAULT FALSE
);

CREATE INDEX ON account(deleted);