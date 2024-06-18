diesel::table! {
    key(id) {
        id -> Int4,
        user_id -> Int4,
        public_key -> Text,
        private_key -> Text
    }
}
