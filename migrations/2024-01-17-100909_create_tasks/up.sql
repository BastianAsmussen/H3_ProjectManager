-- Your SQL goes here
CREATE TABLE tasks
(
    id           SERIAL PRIMARY KEY,
    name         VARCHAR NOT NULL,
    is_completed BOOLEAN NOT NULL DEFAULT FALSE,

    todo_id      INTEGER NOT NULL,

    FOREIGN KEY (todo_id) REFERENCES todos (id) ON DELETE CASCADE
);
