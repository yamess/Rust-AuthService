use diesel::prelude::*;

table! {
    users {
        id -> Uuid,
        email -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}
