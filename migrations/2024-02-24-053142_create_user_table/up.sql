-- Your SQL goes here
CREATE TABLE "users"(
	"id" UUID NOT NULL PRIMARY KEY,
	"email" VARCHAR NOT NULL,
	"first_name" VARCHAR NOT NULL,
	"last_name" VARCHAR NOT NULL,
	"created_at" TIMESTAMP NOT NULL,
	"updated_at" TIMESTAMP
);

