// @generated automatically by Diesel CLI.

diesel::table! {
    adress (id) {
        id -> Integer,
        id_supplier -> Integer,
        cep -> Integer,
        city -> Text,
        neighborhood -> Text,
        block -> Nullable<Text>,
        number -> Text,
        complement -> Nullable<Text>,
    }
}

diesel::table! {
    article (id) {
        id -> Integer,
        id_stock -> Integer,
        id_purchase -> Integer,
        amount -> Integer,
    }
}

diesel::table! {
    category (id) {
        id -> SmallInt,
        name -> Text,
    }
}

diesel::table! {
    product (id) {
        id -> Integer,
        name -> Text,
        price -> Float,
        id_category -> SmallInt,
        total_amount -> Integer,
    }
}

diesel::table! {
    purchase (id) {
        id -> Integer,
        id_user -> Integer,
        time -> Timestamp,
        status -> SmallInt,
    }
}

diesel::table! {
    stock (id) {
        id -> Integer,
        id_product -> Integer,
        id_supplier -> Integer,
        expired -> Bool,
        expire_date -> Timestamp,
        available -> Bool,
        batch -> Nullable<BigInt>,
        amount -> Integer,
    }
}

diesel::table! {
    supplier (id) {
        id -> Integer,
        name -> Text,
        cnpj -> Nullable<Text>,
        cpf -> Nullable<Text>,
        email -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    adress,
    article,
    category,
    product,
    purchase,
    stock,
    supplier,
);
