-- Your SQL goes here


CREATE TABLE "classes"
(
    "id"         UUID      NOT NULL PRIMARY KEY,
    "name"       VARCHAR   NOT NULL,
    "student_id" UUID      NOT NULL,
    "created_at" TIMESTAMP NOT NULL,
    "updated_at" TIMESTAMP,
    FOREIGN KEY ("student_id") REFERENCES "students" ("id")
);

