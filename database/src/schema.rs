// @generated automatically by Diesel CLI.

diesel::table! {
    section (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    product (id) {
        id -> Integer,
        name -> Text,
        total_amount -> Integer,
        measure_unit -> Text,
        measure -> Float,
        section_id -> Integer
    }
}
