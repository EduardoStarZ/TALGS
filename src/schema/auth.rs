// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> SmallInt,
        name -> Text,
        password -> Text,
        group -> Integer,
        email -> Text,
    }
}
