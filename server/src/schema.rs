table! {
    sessions (id) {
        id -> Int4,
        user_id -> Int4,
        token -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        password_hash -> Varchar,
        user_role -> Bpchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(sessions -> users (user_id));

allow_tables_to_appear_in_same_query!(
    sessions,
    users,
);
