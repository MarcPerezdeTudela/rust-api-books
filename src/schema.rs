// @generated automatically by Diesel CLI.

diesel::table! {
    books (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        author -> Varchar,
        year -> Int4,
    }
}
