use diesel::prelude::*;

table! {
    users {
        id -> Uuid,
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    schools {
        id -> Uuid,
        name -> Varchar,
        website -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    students {
        id -> Int4,
        first_name -> VarChar,
        last_name -> VarChar,
        program -> VarChar, // @TODO: Change to enum to avoid discrepancies
        department -> Nullable<VarChar>,
        user_id -> Uuid,
        school_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    classes {
        id -> Int4,
        name -> VarChar,
        student_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    schedules {
        id -> Int4,
        student_id -> Int4,
        class_id -> Int4,
        day_of_week -> Int2,
        start_time -> Timestamp,
        end_time -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

allow_tables_to_appear_in_same_query!(
    users,
    schools,
    students,
    classes,
    schedules,
);

joinable!(students -> users (user_id));
joinable!(students -> schools (school_id));
joinable!(classes -> students (student_id));
joinable!(schedules -> students (student_id));
joinable!(schedules -> classes (class_id));