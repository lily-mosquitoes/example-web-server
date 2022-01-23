table! {
    ifus (id) {
        id -> Int4,
        code -> Varchar,
        file_url -> Text,
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

joinable!(products -> ifus (ifu_id));

allow_tables_to_appear_in_same_query!(
    ifus,
    products,
    users,
);
