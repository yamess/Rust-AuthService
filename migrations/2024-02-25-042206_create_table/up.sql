-- Your SQL goes here
CREATE TABLE "users"
(
    "id"         UUID      NOT NULL PRIMARY KEY,
    "email"      VARCHAR   NOT NULL UNIQUE,
    "password"   VARCHAR   NOT NULL,
    "created_at" TIMESTAMP NOT NULL,
    "updated_at" TIMESTAMP
);

