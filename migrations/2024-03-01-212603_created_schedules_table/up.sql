-- Your SQL goes here




CREATE TABLE "schedules"(
	"id" UUID NOT NULL PRIMARY KEY,
	"student_id" UUID NOT NULL,
	"class_id" UUID NOT NULL,
	"day_of_week" INT2 NOT NULL,
	"start_time" TIMESTAMP NOT NULL,
	"end_time" TIMESTAMP NOT NULL,
	"created_at" TIMESTAMP NOT NULL,
	"updated_at" TIMESTAMP,
	FOREIGN KEY ("student_id") REFERENCES "students"("id"),
	FOREIGN KEY ("class_id") REFERENCES "classes"("id")
);

