CREATE TYPE role AS ENUM ('basic', 'root');

CREATE TABLE account (
    id          serial PRIMARY KEY,
    name        text NOT NULL UNIQUE,
    email       text NOT NULL UNIQUE,
    password    text NOT NULL,
    role        role NOT NULL DEFAULT 'basic',
    created     timestamptz NOT NULL DEFAULT NOW(),
    deleted     timestamptz
);

CREATE TABLE thing (
    id          serial PRIMARY KEY,
    name        text NOT NULL UNIQUE,
    file        text NOT NULL,
    created     timestamptz NOT NULL DEFAULT NOW(),
    deleted     timestamptz
);

CREATE TABLE category (
    id          serial PRIMARY KEY,
    name        text NOT NULL UNIQUE,
    created     timestamptz NOT NULL DEFAULT NOW(),
    deleted     timestamptz
);

CREATE TABLE score(
    thing_id        integer NOT NULL REFERENCES thing(id),
    category_id     integer NOT NULL REFERENCES category(id),
    score           integer NOT NULL DEFAULT 0
);

CREATE INDEX ON account(name);
CREATE INDEX ON account(email);
CREATE INDEX ON account(role);
CREATE INDEX ON account(created);
CREATE INDEX ON account(deleted);

CREATE INDEX ON thing(name);
CREATE INDEX ON thing(created);
CREATE INDEX ON thing(deleted);

CREATE INDEX ON category(name);
CREATE INDEX ON category(created);
CREATE INDEX ON category(deleted);

CREATE INDEX ON score(thing_id);
CREATE INDEX ON score(category_id);
CREATE INDEX ON score(score);
