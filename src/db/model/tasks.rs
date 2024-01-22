use diesel::{Associations, BelongingToDsl, ExpressionMethods, Identifiable, Insertable, PgConnection, Queryable, QueryDsl, QueryResult, RunQueryDsl, Selectable, SelectableHelper};
use crate::db::schema::tasks;

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::db::schema::todos)]
pub struct Todo {
    pub id: i32,
    pub name: String,
}

impl Todo {
    pub fn new(conn: &mut PgConnection, name: &str) -> QueryResult<Self> {
        diesel::insert_into(crate::db::schema::todos::table)
            .values(crate::db::schema::todos::name.eq(name))
            .returning(Self::as_select())
            .get_result(conn)
    }
    
    pub fn by_id(conn: &mut PgConnection, id: i32) -> QueryResult<Self> {
        crate::db::schema::todos::table
            .find(id)
            .select(Self::as_select())
            .first(conn)
    }

    pub fn by_name(conn: &mut PgConnection, name: &str) -> QueryResult<Self> {
        crate::db::schema::todos::table
            .filter(crate::db::schema::todos::name.eq(name))
            .select(Self::as_select())
            .first(conn)
    }

    pub fn tasks(&self, conn: &mut PgConnection) -> QueryResult<Vec<Task>> {
        Task::belonging_to(self)
            .select(Task::as_select())
            .load(conn)
    }

    pub fn add_task(&self, conn: &mut PgConnection, name: &str) -> QueryResult<Task> {
        diesel::insert_into(tasks::table)
            .values(NewTask {
                name,
                is_completed: false,
                todo_id: self.id,
            })
            .returning(Task::as_select())
            .get_result(conn)
    }
    
    pub fn complete_task(&self, conn: &mut PgConnection, name: &str) -> QueryResult<Task> {
        diesel::update(tasks::table)
            .filter(tasks::name.eq(name))
            .set(tasks::is_completed.eq(true))
            .returning(Task::as_select())
            .get_result(conn)
    }
}

#[derive(Debug, Eq, PartialEq, Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name = crate::db::schema::tasks)]
#[diesel(belongs_to(Todo))]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub is_completed: bool,
    pub todo_id: i32,
}

impl Task {
    pub fn new(conn: &mut PgConnection, name: &str, is_completed: bool, todo_id: i32) -> QueryResult<Self> {
        diesel::insert_into(tasks::table)
            .values(NewTask {
                name,
                is_completed,
                todo_id,
            })
            .returning(Self::as_select())
            .get_result(conn)
    }

    pub fn by_id(conn: &mut PgConnection, id: i32) -> QueryResult<Self> {
        tasks::dsl::tasks
            .find(id)
            .select(Self::as_select())
            .first(conn)
    }

    pub fn by_name(conn: &mut PgConnection, name: &str) -> QueryResult<Self> {
        tasks::dsl::tasks
            .filter(tasks::name.eq(name))
            .select(Self::as_select())
            .first(conn)
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::db::schema::tasks)]
struct NewTask<'a> {
    name: &'a str,
    is_completed: bool,
    todo_id: i32,
}