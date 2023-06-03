// @generated automatically by Diesel CLI.

diesel::table! {
    pastes (id) {
        id -> Int4,
        owner -> Int4,
        filename -> Text,
        mime -> Text,
        link -> Text,
        #[sql_name = "type"]
        type_ -> Text,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        image -> Nullable<Text>,
        password_hash -> Text,
    }
}

diesel::joinable!(pastes -> users (owner));

diesel::allow_tables_to_appear_in_same_query!(
    pastes,
    users,
);
