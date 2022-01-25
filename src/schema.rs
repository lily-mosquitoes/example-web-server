table! {
    files (id) {
        id -> Uuid,
        name -> Varchar,
        content_type -> Varchar,
        size -> Int8,
        data -> Bytea,
    }
}

table! {
    ifus (id) {
        id -> Int4,
        code -> Varchar,
        file_id -> Nullable<Uuid>,
    }
}

table! {
    products (id) {
        id -> Int4,
        code -> Varchar,
        name -> Text,
        ifu_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password_hash -> Varchar,
        last_login -> Timestamptz,
        admin_status -> Bool,
    }
}

joinable!(ifus -> files (file_id));
joinable!(products -> ifus (ifu_id));

allow_tables_to_appear_in_same_query!(
    files,
    ifus,
    products,
    users,
);
