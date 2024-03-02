-- Your SQL goes here

ALTER TABLE "schedules"
    DROP COLUMN "start_time";
ALTER TABLE "schedules"
    DROP COLUMN "end_time";
ALTER TABLE "schedules"
    ADD COLUMN "start_time" TIME NOT NULL;
ALTER TABLE "schedules"
    ADD COLUMN "end_time" TIME NOT NULL;




