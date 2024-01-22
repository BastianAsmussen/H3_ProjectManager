// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Int4,
        name -> Varchar,
        is_completed -> Bool,
        todo_id -> Int4,
    }
}

diesel::table! {
    teams (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        todo_id -> Nullable<Int4>,
    }
}

diesel::table! {
    teams_workers (team_id, worker_id) {
        team_id -> Int4,
        worker_id -> Int4,
    }
}

diesel::table! {
    todos (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    workers (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        task_id -> Nullable<Int4>,
    }
}

diesel::joinable!(tasks -> todos (todo_id));
diesel::joinable!(teams -> todos (todo_id));
diesel::joinable!(teams_workers -> teams (team_id));
diesel::joinable!(teams_workers -> workers (worker_id));
diesel::joinable!(workers -> tasks (task_id));

diesel::allow_tables_to_appear_in_same_query!(
    tasks,
    teams,
    teams_workers,
    todos,
    workers,
);
