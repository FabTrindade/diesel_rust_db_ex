// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 100]
        user_name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
    }
}
