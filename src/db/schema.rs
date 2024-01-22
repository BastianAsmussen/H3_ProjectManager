// @generated automatically by Diesel CLI.

diesel::table! {
    teams (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    teams_workers (team_id, worker_id) {
        team_id -> Int4,
        worker_id -> Int4,
    }
}

diesel::table! {
    workers (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::joinable!(teams_workers -> teams (team_id));
diesel::joinable!(teams_workers -> workers (worker_id));

diesel::allow_tables_to_appear_in_same_query!(
    teams,
    teams_workers,
    workers,
);
