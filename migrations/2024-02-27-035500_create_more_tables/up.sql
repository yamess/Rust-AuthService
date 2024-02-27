-- Your SQL goes here
ALTER TABLE "schedules" ADD COLUMN "class_id" INT4 NOT NULL;

ALTER TABLE "schools" DROP COLUMN "website";
ALTER TABLE "schools" ADD COLUMN "website" VARCHAR NOT NULL;

ALTER TABLE "students" DROP COLUMN "department";
ALTER TABLE "students" ADD COLUMN "department" VARCHAR;


CREATE TABLE "classes"(
	"id" INT4 NOT NULL PRIMARY KEY,
	"name" VARCHAR NOT NULL,
	"student_id" INT4 NOT NULL,
	"created_at" TIMESTAMP NOT NULL,
	"updated_at" TIMESTAMP,
	FOREIGN KEY ("student_id") REFERENCES "students"("id")
);

