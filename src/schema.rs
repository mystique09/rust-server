table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Varchar>,
        author -> Uuid,
        image -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        email -> Varchar,
        role -> Varchar,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
