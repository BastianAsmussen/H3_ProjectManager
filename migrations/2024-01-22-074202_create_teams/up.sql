-- Your SQL goes here
CREATE TABLE teams(
    id              SERIAL PRIMARY KEY,
    name            VARCHAR(255) NOT NULL,

    todo_id INTEGER REFERENCES todos(id) ON DELETE SET NULL
);
