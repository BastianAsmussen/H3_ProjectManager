use diesel::{Identifiable, Insertable, PgConnection, Queryable, QueryResult, RunQueryDsl, Selectable};
use crate::db::schema::{teams, workers, teams_workers};

#[derive(Debug, Eq, PartialEq, Queryable, Selectable, Identifiable)]
#[diesel(table_name = teams)]
pub struct Team {
    pub id: i32,
    pub name: String,
}

impl Team {
    pub fn new(conn: &mut PgConnection, name: &str) -> QueryResult<Self> {
        let new_team = NewTeam { name };

        diesel::insert_into(teams::dsl::teams)
            .values(&new_team)
            .get_result(conn)
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = teams)]
struct NewTeam<'a> {
    pub name: &'a str,
}

#[derive(Debug, Eq, PartialEq, Queryable, Selectable, Identifiable)]
#[diesel(table_name = workers)]
pub struct Worker {
    pub id: i32,
    pub name: String,
}

impl Worker {
    pub fn new(conn: &mut PgConnection, name: &str) -> QueryResult<Self> {
        let new_worker = NewWorker { name };

        diesel::insert_into(workers::dsl::workers)
            .values(&new_worker)
            .get_result(conn)
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = workers)]
struct NewWorker<'a> {
    pub name: &'a str,
}

#[derive(Debug, Eq, PartialEq, Queryable, Selectable, Identifiable)]
#[diesel(table_name = teams_workers)]
#[diesel(belongs_to(Team))]
#[diesel(belongs_to(Worker))]
#[diesel(primary_key(team_id, worker_id))]
pub struct TeamWorker {
    pub team_id: i32,
    pub worker_id: i32,
}

impl TeamWorker {
    pub fn new(conn: &mut PgConnection, team_id: i32, worker_id: i32) -> QueryResult<Self> {
        let new_team_worker = NewTeamWorker { team_id, worker_id };

        diesel::insert_into(teams_workers::dsl::teams_workers)
            .values(&new_team_worker)
            .get_result(conn)
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = teams_workers)]
struct NewTeamWorker {
    pub team_id: i32,
    pub worker_id: i32,
}
