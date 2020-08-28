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
