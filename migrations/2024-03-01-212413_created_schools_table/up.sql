-- Your SQL goes here

CREATE TABLE "schools"
(
    "id"         UUID      NOT NULL PRIMARY KEY,
    "name"       VARCHAR   NOT NULL UNIQUE,
    "website"    VARCHAR   NOT NULL UNIQUE,
    "created_at" TIMESTAMP NOT NULL,
    "updated_at" TIMESTAMP
);

