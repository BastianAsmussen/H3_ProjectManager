use crate::db::schema::tasks::dsl::tasks;
use crate::db::schema::tasks::is_completed;
use crate::db::schema::todos::dsl::todos;
use diesel::{
    Associations, BelongingToDsl, ExpressionMethods, Identifiable, Insertable, PgConnection,
    QueryDsl, QueryResult, Queryable, RunQueryDsl, Selectable, SelectableHelper,
};
use std::fmt::Display;

#[derive(Debug, Eq, PartialEq, Queryable, Identifiable, Selectable, Insertable)]
#[diesel(table_name = crate::db::schema::todos)]
pub struct Todo {
    pub id: i32,
    pub name: String,
}

impl Todo {
    pub fn new(conn: &mut PgConnection, name: &str) -> QueryResult<Self> {
        let new_todo = NewTodo { name };

        diesel::insert_into(todos)
            .values(&new_todo)
            .get_result(conn)
    }

    pub fn get_tasks(&self, conn: &mut PgConnection) -> QueryResult<Vec<Task>> {
        Task::belonging_to(self)
            .select(Task::as_select())
            .load(conn)
    }

    pub fn add_task(&self, conn: &mut PgConnection, name: &str) -> QueryResult<Task> {
        Task::new(conn, self.id, name)
    }
}

impl Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:", self.name)?;

        Ok(())
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::db::schema::todos)]
struct NewTodo<'a> {
    pub name: &'a str,
}

#[derive(Debug, Eq, PartialEq, Queryable, Identifiable, Selectable, Associations)]
#[diesel(belongs_to(Todo))]
#[diesel(table_name = crate::db::schema::tasks)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub is_completed: bool,
    pub todo_id: i32,
}

impl Task {
    pub fn new(conn: &mut PgConnection, todo_id: i32, name: &str) -> QueryResult<Self> {
        let new_task = NewTask { todo_id, name };

        diesel::insert_into(crate::db::schema::tasks::table)
            .values(&new_task)
            .get_result(conn)
    }

    pub fn complete(&mut self, conn: &mut PgConnection) -> QueryResult<()> {
        self.is_completed = true;

        diesel::update(tasks.find(self.id))
            .set(is_completed.eq(self.is_completed))
            .execute(conn)?;

        Ok(())
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::db::schema::tasks)]
struct NewTask<'a> {
    pub todo_id: i32,
    pub name: &'a str,
}
