CREATE TYPE role AS ENUM ('basic', 'root');

CREATE TABLE account (
    id          serial PRIMARY KEY,
    name        text NOT NULL UNIQUE,
    email       text NOT NULL UNIQUE,
    password    text NOT NULL,
    role        role NOT NULL DEFAULT 'basic',
    deleted     timestamptz
);

CREATE TABLE thing (
    id          serial PRIMARY KEY,
    name        text NOT NULL,
    file        text NOT NULL,
    deleted     timestamptz
);

CREATE TABLE category (
    id          serial PRIMARY KEY,
    name        text NOT NULL,
    deleted     timestamptz
);

CREATE TABLE rank(
    thing_id        integer NOT NULL REFERENCES thing(id),
    category_id     integer NOT NULL REFERENCES category(id),
    score           double precision NOT NULL DEFAULT 0.0,
    run             integer NOT NULL,
    shuffle         real NOT NULL DEFAULT RANDOM()
);

CREATE TABLE poll (
    account_id      integer NOT NULL REFERENCES account(id) UNIQUE,
    category_id     integer NOT NULL REFERENCES category(id),
    thing_id_a      integer NOT NULL REFERENCES thing(id),
    thing_id_b      integer NOT NULL REFERENCES thing(id)
);

CREATE INDEX ON account(name);
CREATE INDEX ON account(email);
CREATE INDEX ON account(role);
CREATE INDEX ON account(deleted);

CREATE INDEX ON thing(name);
CREATE INDEX ON thing(deleted);

CREATE INDEX ON category(name);
CREATE INDEX ON category(deleted);

CREATE INDEX ON rank(thing_id);
CREATE INDEX ON rank(category_id);
CREATE INDEX ON rank(score);
CREATE INDEX ON rank(run, shuffle);