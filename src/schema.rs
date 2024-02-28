// @generated automatically by Diesel CLI.

diesel::table! {
    classes (id) {
        id -> Int4,
        name -> Varchar,
        student_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    schedules (id) {
        id -> Int4,
        student_id -> Int4,
        start_time -> Timestamp,
        end_time -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        class_id -> Int4,
        day_of_week -> Int2,
    }
}

diesel::table! {
    schools (id) {
        id -> Uuid,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        website -> Varchar,
    }
}

diesel::table! {
    students (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        program -> Varchar,
        user_id -> Uuid,
        school_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        department -> Nullable<Varchar>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(classes -> students (student_id));
diesel::joinable!(schedules -> students (student_id));
diesel::joinable!(students -> schools (school_id));
diesel::joinable!(students -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(classes, schedules, schools, students, users,);
