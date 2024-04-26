// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Text,
        username -> Text,
        email -> Text,
        password -> Text,
        secret -> Nullable<Text>,
        flags -> Integer,
    }
}
