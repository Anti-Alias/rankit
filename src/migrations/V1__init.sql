CREATE TYPE role AS ENUM ('basic', 'admin', 'root');

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
    shuffle         real NOT NULL DEFAULT RANDOM(),
    UNIQUE (thing_id, category_id   )
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

COMMENT ON TYPE role IS
'The set of permissions an account has.'
'"basic" accounts can only submit polls.'
'"admin" accounts inherit from "basic", and can POST new "things", "categories" and "ranks".'
'"root" accounts inherit from "admin", can can give an admin role to accounts.'
'There is only 1 root user.';

COMMENT ON TABLE account IS
'A user that is responsible for answering "polls" on the site.'
'May have other privileges depending on their "role".';

COMMENT ON TABLE thing IS
'A "thing" to be voted on within a paricular "category".'
'May belong to 0 or more "categories".';

COMMENT ON TABLE category IS
'A category where "things" may belong to.'
'May be used to describe 0 or more "things".';

COMMENT ON TABLE rank IS
'Juncture table that assigns "things" to "categories".'
'Also, assigns an ELO score to things within those categories.'
'The columns "run" and "shuffle" are state variables used in selecting two random "things" in the voting algorithm.';

COMMENT ON TABLE poll IS
'Stores the "polling state" of accounts.'
'Represents a comparison of two "things" within a "category" both things belong to.'
'A single account can have 0 or 1 polls.'
'When an account not in a "polling state" starts a poll, a new entry is inserted, putting the account in a polling state.'
'When an account in a "polling state" starts a poll, an existing entry is overwritten.'
'When an account in a "polling state" ends a poll, an existing entry is removed, taking an accountn of a polling state.'
;
