CREATE TABLE "users"(
    "id" UUID NOT NULL,
    "email" VARCHAR(255) NOT NULL,
    "password" VARCHAR(255) NOT NULL,
    "created_at" TIMESTAMP(0) WITHOUT TIME ZONE NOT NULL,
    "updated_at" TIMESTAMP(0) WITHOUT TIME ZONE NULL
);
ALTER TABLE
    "users" ADD PRIMARY KEY("id");
ALTER TABLE
    "users" ADD CONSTRAINT "users_email_unique" UNIQUE("email");
CREATE TABLE "students"(
    "id" BIGINT NOT NULL,
    "first_name" VARCHAR(255) NOT NULL,
    "last_name" VARCHAR(255) NOT NULL,
    "program" VARCHAR(255) NOT NULL,
    "department" VARCHAR(255) NULL,
    "school_id" UUID NOT NULL,
    "user_id" UUID NOT NULL,
    "created_at" TIMESTAMP(0) WITHOUT TIME ZONE NULL,
    "updated_at" TIMESTAMP(0) WITHOUT TIME ZONE NOT NULL
);
ALTER TABLE
    "students" ADD PRIMARY KEY("id");
ALTER TABLE
    "students" ADD CONSTRAINT "students_user_id_unique" UNIQUE("user_id");
CREATE TABLE "schools"(
    "id" UUID NOT NULL,
    "name" VARCHAR(255) NOT NULL,
    "website" VARCHAR(255) NOT NULL,
    "created_at" TIMESTAMP(0) WITHOUT TIME ZONE NOT NULL,
    "updated_at" TIMESTAMP(0) WITHOUT TIME ZONE NOT NULL
);
ALTER TABLE
    "schools" ADD PRIMARY KEY("id");
ALTER TABLE
    "schools" ADD CONSTRAINT "schools_name_unique" UNIQUE("name");
ALTER TABLE
    "schools" ADD CONSTRAINT "schools_website_unique" UNIQUE("website");
CREATE TABLE "classes"(
    "id" BIGINT NOT NULL,
    "name" VARCHAR(255) NOT NULL,
    "student_id" BIGINT NOT NULL,
    "created_at" TIMESTAMP(0) WITHOUT TIME ZONE NOT NULL,
    "updated_at" TIMESTAMP(0) WITHOUT TIME ZONE NOT NULL
);
ALTER TABLE
    "classes" ADD PRIMARY KEY("id");
CREATE TABLE "schedules"(
    "id" BIGINT NOT NULL,
    "student_id" BIGINT NOT NULL,
    "class_id" BIGINT NOT NULL,
    "day_of_week" INTEGER NOT NULL,
    "start_time" TIMESTAMP(0) WITHOUT TIME ZONE NOT NULL,
    "end_time" TIMESTAMP(0) WITHOUT TIME ZONE NOT NULL,
    "created_at" TIMESTAMP(0) WITHOUT TIME ZONE NOT NULL,
    "updated_at" TIMESTAMP(0) WITHOUT TIME ZONE NOT NULL
);
ALTER TABLE
    "schedules" ADD PRIMARY KEY("id");
ALTER TABLE
    "students" ADD CONSTRAINT "students_user_id_foreign" FOREIGN KEY("user_id") REFERENCES "users"("id");
ALTER TABLE
    "schedules" ADD CONSTRAINT "schedules_class_id_foreign" FOREIGN KEY("class_id") REFERENCES "classes"("id");
ALTER TABLE
    "schedules" ADD CONSTRAINT "schedules_student_id_foreign" FOREIGN KEY("student_id") REFERENCES "students"("id");
ALTER TABLE
    "classes" ADD CONSTRAINT "classes_student_id_foreign" FOREIGN KEY("student_id") REFERENCES "students"("id");
ALTER TABLE
    "students" ADD CONSTRAINT "students_school_id_foreign" FOREIGN KEY("school_id") REFERENCES "schools"("id");