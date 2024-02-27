-- This file should undo anything in `up.sql`
ALTER TABLE "schedules" DROP COLUMN "class_id";

ALTER TABLE "schools" DROP COLUMN "website";
ALTER TABLE "schools" ADD COLUMN "website" VARCHAR;

ALTER TABLE "students" DROP COLUMN "department";
ALTER TABLE "students" ADD COLUMN "department" VARCHAR NOT NULL;


DROP TABLE IF EXISTS "classes";
