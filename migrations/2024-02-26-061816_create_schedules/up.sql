-- Your SQL goes here



CREATE TABLE "schedules"(
	"id" INT4 NOT NULL PRIMARY KEY,
	"student_id" INT4 NOT NULL,
	"start_time" TIMESTAMP NOT NULL,
	"end_time" TIMESTAMP NOT NULL,
	"created_at" TIMESTAMP NOT NULL,
	"updated_at" TIMESTAMP,
	FOREIGN KEY ("student_id") REFERENCES "students"("id")
);

