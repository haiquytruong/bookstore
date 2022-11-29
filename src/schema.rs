// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        password -> Text,
        phone -> Varchar,
        first_name -> Text,
        last_name -> Text,
        birthday -> Date,
        activated -> Nullable<Bool>,
    }
}
