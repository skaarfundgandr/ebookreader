// @generated automatically by Diesel CLI.

diesel::table! {
    configuration (configuration_id) {
        configuration_id -> Integer,
        book_path -> Nullable<Text>,
    }
}
