-- Your SQL goes here
CREATE TABLE teams_workers(
    team_id   INTEGER REFERENCES teams(id)   ON DELETE CASCADE,
    worker_id INTEGER REFERENCES workers(id) ON DELETE CASCADE,

    PRIMARY KEY (team_id, worker_id)
);
