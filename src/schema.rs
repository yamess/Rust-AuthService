// @generated automatically by Diesel CLI.

diesel::table! {
    classes (id) {
        id -> Uuid,
        name -> Varchar,
        student_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    schedules (id) {
        id -> Uuid,
        student_id -> Uuid,
        class_id -> Uuid,
        day_of_week -> Int2,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        start_time -> Time,
        end_time -> Time,
    }
}

diesel::table! {
    schools (id) {
        id -> Uuid,
        name -> Varchar,
        website -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    students (id) {
        id -> Uuid,
        first_name -> Varchar,
        last_name -> Varchar,
        program -> Varchar,
        department -> Nullable<Varchar>,
        user_id -> Uuid,
        school_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        password -> Varchar,
        is_active -> Bool,
        is_admin -> Bool,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(classes -> students (student_id));
diesel::joinable!(schedules -> classes (class_id));
diesel::joinable!(schedules -> students (student_id));
diesel::joinable!(students -> schools (school_id));
diesel::joinable!(students -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(classes, schedules, schools, students, users,);
