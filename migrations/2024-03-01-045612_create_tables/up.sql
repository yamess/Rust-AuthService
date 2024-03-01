-- Your SQL goes here
CREATE TABLE "users"
(
    "id"         UUID      NOT NULL PRIMARY KEY,
    "email"      VARCHAR   NOT NULL UNIQUE CHECK (email ~* '^.+@.+\..+$'),
    "password"   VARCHAR   NOT NULL CHECK (LENGTH(password) >= 8),
    "is_active"  BOOL      NOT NULL DEFAULT TRUE,
    "is_admin"   BOOL      NOT NULL DEFAULT FALSE,
    "created_at" TIMESTAMP NOT NULL,
    "updated_at" TIMESTAMP
);

CREATE TABLE "schools"
(
    "id"         UUID      NOT NULL PRIMARY KEY,
    "name"       VARCHAR   NOT NULL UNIQUE,
    "website"    VARCHAR   NOT NULL UNIQUE CHECK (website ~* '^(http|https)://.+\..+$'),
    "created_at" TIMESTAMP NOT NULL,
    "updated_at" TIMESTAMP
);

CREATE TABLE "students"
(
    "id"         INT4      NOT NULL PRIMARY KEY,
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

CREATE TABLE "classes"
(
    "id"         INT4      NOT NULL PRIMARY KEY,
    "name"       VARCHAR   NOT NULL,
    "student_id" INT4      NOT NULL,
    "created_at" TIMESTAMP NOT NULL,
    "updated_at" TIMESTAMP,
    FOREIGN KEY ("student_id") REFERENCES "students" ("id")
);

CREATE TABLE "schedules"
(
    "id"          INT4      NOT NULL PRIMARY KEY,
    "student_id"  INT4      NOT NULL,
    "class_id"    INT4      NOT NULL,
    "day_of_week" INT2      NOT NULL,
    "start_time"  TIMESTAMP NOT NULL CHECK (start_time < end_time),
    "end_time"    TIMESTAMP NOT NULL CHECK (end_time > start_time),
    "created_at"  TIMESTAMP NOT NULL,
    "updated_at"  TIMESTAMP,
    FOREIGN KEY ("student_id") REFERENCES "students" ("id"),
    FOREIGN KEY ("class_id") REFERENCES "classes" ("id")
);
