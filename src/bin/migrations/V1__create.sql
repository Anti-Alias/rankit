CREATE TYPE role AS ENUM ('root', 'basic');
CREATE TABLE account (
    id      serial NOT NULL,
    name    text NOT NULL,
    role    role NOT NULL DEFAULT 'basic'
);