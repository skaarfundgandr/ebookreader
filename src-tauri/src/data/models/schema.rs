// @generated automatically by Diesel CLI.

diesel::table! {
    configuration (configuration_id) {
        configuration_id -> Integer,
        book_path -> Nullable<Text>,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Integer,
        username -> Text,
        email -> Text,
        password_hash -> Text,
        created_at -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(configuration, users,);
