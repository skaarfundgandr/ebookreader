// @generated automatically by Diesel CLI.

diesel::table! {
    authors (author_id) {
        author_id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    books (id) {
        id -> Integer,
        title -> Text,
        author_id -> Nullable<Integer>,
        published_date -> Nullable<Text>,
        publisher_id -> Nullable<Integer>,
        isbn -> Nullable<Text>,
        file_type -> Nullable<Text>,
        file_path -> Text,
        added_at -> Nullable<Text>,
    }
}

diesel::table! {
    configuration (configuration_id) {
        configuration_id -> Integer,
        book_path -> Nullable<Text>,
    }
}

diesel::table! {
    publishers (publisher_id) {
        publisher_id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    user_library (user_id, book_id) {
        user_id -> Integer,
        book_id -> Integer,
        added_at -> Nullable<Text>,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Integer,
        username -> Text,
        email -> Text,
        password_hash -> Text,
        created_at -> Nullable<Text>,
    }
}

diesel::joinable!(books -> authors (author_id));
diesel::joinable!(books -> publishers (publisher_id));
diesel::joinable!(user_library -> books (book_id));
diesel::joinable!(user_library -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    authors,
    books,
    configuration,
    publishers,
    user_library,
    users,
);
