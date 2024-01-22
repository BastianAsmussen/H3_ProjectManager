use diesel::{Associations, ExpressionMethods, Identifiable, Insertable, PgConnection, Queryable, QueryDsl, QueryResult, RunQueryDsl, Selectable, SelectableHelper};
use crate::db::schema::{teams, teams_workers, workers};

#[derive(Debug, Eq, PartialEq, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::db::schema::teams)]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub todo_id: Option<i32>,
}

impl Team {
    pub fn new(conn: &mut PgConnection, name: &str, todo_id: i32) -> QueryResult<Self> {
        diesel::insert_into(teams::table)
            .values(NewTeam {
                name,
                todo_id,
            })
            .returning(Self::as_select())
            .get_result(conn)
    }
    
    pub fn by_id(conn: &mut PgConnection, id: i32) -> QueryResult<Self> {
        teams::table
            .find(id)
            .select(Self::as_select())
            .first(conn)
    }
    
    pub fn by_name(conn: &mut PgConnection, name: &str) -> QueryResult<Self> {
        teams::table
            .filter(teams::name.eq(name))
            .select(Self::as_select())
            .first(conn)
    }

    pub fn by_no_tasks(conn: &mut PgConnection) -> QueryResult<Vec<Self>> {
        teams::table
            .filter(teams::todo_id.is_null())
            .select(Self::as_select())
            .load(conn)
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::db::schema::teams)]
struct NewTeam<'a> {
    name: &'a str,
    todo_id: i32,
}

#[derive(Debug, Eq, PartialEq, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::db::schema::workers)]
pub struct Worker {
    pub id: i32,
    pub name: String,
    pub task_id: Option<i32>,
}

impl Worker {
    pub fn new(conn: &mut PgConnection, name: &str, task_id: i32) -> QueryResult<Self> {
        diesel::insert_into(workers::table)
            .values(NewWorker {
                name,
                task_id,
            })
            .returning(Self::as_select())
            .get_result(conn)
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::db::schema::workers)]
struct NewWorker<'a> {
    name: &'a str,
    task_id: i32,
}

#[derive(Debug, Eq, PartialEq, Queryable, Selectable, Insertable, Identifiable, Associations)]
#[diesel(table_name = crate::db::schema::teams_workers)]
#[diesel(primary_key(team_id, worker_id))]
#[diesel(belongs_to(Team))]
#[diesel(belongs_to(Worker))]
pub struct TeamWorker {
    pub team_id: i32,
    pub worker_id: i32,
}

impl TeamWorker {
    pub fn new(conn: &mut PgConnection, team_id: i32, worker_id: i32) -> QueryResult<Self> {
        diesel::insert_into(teams_workers::table)
            .values(Self {
                team_id,
                worker_id,
            })
            .returning(Self::as_select())
            .get_result(conn)
    }
}
