// @generated automatically by Diesel CLI.

diesel::table! {
    jobs (id) {
        id -> Integer,
        created -> Text,
        company_name -> Text,
        title_id -> Integer,
        status_id -> Integer,
        link -> Nullable<Text>,
        notes -> Nullable<Text>,
        sprint_id -> Integer,
    }
}

diesel::table! {
    sprints (id) {
        id -> Integer,
        name -> Text,
        start_date -> Text,
        end_date -> Nullable<Text>,
        num_jobs -> Integer,
    }
}

diesel::table! {
    statuses (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    titles (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::joinable!(jobs -> sprints (sprint_id));
diesel::joinable!(jobs -> statuses (status_id));
diesel::joinable!(jobs -> titles (title_id));

diesel::allow_tables_to_appear_in_same_query!(jobs, sprints, statuses, titles,);
