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
    account_id  integer NOT NULL REFERENCES account(id),
    name        text NOT NULL,
    file        text NOT NULL,
    created     timestamptz NOT NULL DEFAULT NOW(),
    deleted     timestamptz,
    UNIQUE      (account_id, name)
);

CREATE TABLE category (
    id          serial PRIMARY KEY,
    account_id  integer NOT NULL REFERENCES account(id),
    name        text NOT NULL
);

CREATE TABLE thing_category(
    thing_id        integer NOT NULL REFERENCES thing(id),
    category_id     integer NOT NULL REFERENCES category(id),
    score           integer NOT NULL DEFAULT 0
);

CREATE INDEX ON account(deleted);
CREATE INDEX ON thing(account_id);
CREATE INDEX ON thing(name);
CREATE INDEX ON thing_category(thing_id);
CREATE INDEX ON thing_category(category_id);