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
        website -> Nullable<Varchar>,
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
        department -> VarChar,
        user_id -> Uuid,
        school_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    schedules {
        id -> Int4,
        student_id -> Int4,
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
    // schedules,
);

allow_tables_to_appear_in_same_query!(students, schedules);

joinable!(students -> users (user_id));
joinable!(students -> schools (school_id));
joinable!(schedules -> students (student_id));