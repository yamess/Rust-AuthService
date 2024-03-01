-- Your SQL goes here
CREATE TABLE "users"(
	"id" UUID NOT NULL PRIMARY KEY,
	"email" VARCHAR NOT NULL UNIQUE CHECK (email ~* '^.+@.+\..+$'),
	"password" VARCHAR NOT NULL CHECK (LENGTH(password) > 8),
	"is_active" BOOL NOT NULL,
	"is_admin" BOOL NOT NULL,
	"created_at" TIMESTAMP NOT NULL,
	"updated_at" TIMESTAMP
);

