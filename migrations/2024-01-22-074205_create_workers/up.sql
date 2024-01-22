-- Your SQL goes here
CREATE TABLE workers(
    id              SERIAL PRIMARY KEY,
    name            VARCHAR(255) NOT NULL,

    task_id INTEGER REFERENCES tasks(id) ON DELETE SET NULL
);
