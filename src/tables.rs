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

// table! {
//     profiles {
//         id -> i32,
//         user_id -> Uuid,
//         first_name -> VarChar,
//         last_name -> VarChar,
//         created_at -> Timestamp,
//         updated_at -> Nullable<Timestamp>,
//     }
// }
