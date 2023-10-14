// @generated automatically by Diesel CLI.

diesel::table! {
    job_data (id) {
        id -> Nullable<Integer>,
        company -> Text,
        date_added -> Text,
        link -> Nullable<Text>,
        notes -> Nullable<Text>,
        status -> Text,
        stint -> Nullable<Integer>,
        title -> Text,
    }
}

diesel::table! {
    stints (id) {
        id -> Nullable<Integer>,
        date_added -> Text,
        stint -> Text,
    }
}

diesel::joinable!(job_data -> stints (stint));

diesel::allow_tables_to_appear_in_same_query!(job_data, stints,);
