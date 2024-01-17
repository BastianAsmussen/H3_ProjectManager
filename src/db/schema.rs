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
    todos (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::joinable!(tasks -> todos (todo_id));

diesel::allow_tables_to_appear_in_same_query!(tasks, todos,);
