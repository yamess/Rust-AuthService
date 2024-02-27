-- Your SQL goes here

CREATE TABLE "schools"
(
    "id"         UUID      NOT NULL PRIMARY KEY,
    "name"       VARCHAR   NOT NULL,
    "website"    VARCHAR   NOT NULL UNIQUE,
    "created_at" TIMESTAMP NOT NULL,
    "updated_at" TIMESTAMP
);

CREATE TABLE "students"
(
    "id"         INT4      NOT NULL PRIMARY KEY,
    "first_name" VARCHAR   NOT NULL,
    "last_name"  VARCHAR   NOT NULL,
    "program"    VARCHAR   NOT NULL,
    "department" VARCHAR   NOT NULL,
    "user_id"    UUID      NOT NULL,
    "school_id"  UUID      NOT NULL,
    "created_at" TIMESTAMP NOT NULL,
    "updated_at" TIMESTAMP,
    FOREIGN KEY ("user_id") REFERENCES "users" ("id"),
    FOREIGN KEY ("school_id") REFERENCES "schools" ("id")
);

