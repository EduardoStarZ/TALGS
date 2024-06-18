// @generated automatically by Diesel CLI.

diesel::table! {
    key (id) {
        id -> Integer,
        user_id -> Integer,
        public_key -> Text,
        private_key -> Text,
    }
}
