-- This file should undo anything in `up.sql`

ALTER TABLE "schedules" DROP COLUMN "start_time";
ALTER TABLE "schedules" DROP COLUMN "end_time";
ALTER TABLE "schedules" ADD COLUMN "start_time" TIMESTAMP NOT NULL;
ALTER TABLE "schedules" ADD COLUMN "end_time" TIMESTAMP NOT NULL;




