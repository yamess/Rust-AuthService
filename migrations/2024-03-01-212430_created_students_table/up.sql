-- Your SQL goes here


CREATE TABLE "students"
(
    "id"         UUID      NOT NULL PRIMARY KEY,
    "first_name" VARCHAR   NOT NULL,
    "last_name"  VARCHAR   NOT NULL,
    "program"    VARCHAR   NOT NULL,
    "department" VARCHAR,
    "user_id"    UUID      NOT NULL UNIQUE,
    "school_id"  UUID      NOT NULL,
    "created_at" TIMESTAMP NOT NULL,
    "updated_at" TIMESTAMP,
    FOREIGN KEY ("user_id") REFERENCES "users" ("id"),
    FOREIGN KEY ("school_id") REFERENCES "schools" ("id")
);

