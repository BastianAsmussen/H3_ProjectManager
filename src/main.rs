use crate::db::model::Todo;
use color_eyre::Result;
use diesel::PgConnection;

mod db;

fn main() -> Result<()> {
    let mut conn = db::get_connection()?;

    let todo = seed_todo(&mut conn)?;

    println!("{}:", todo.name);
    todo.get_tasks(&mut conn)?
        .iter()
        .for_each(|task| println!("- {}", task.name));

    Ok(())
}

fn seed_todo(conn: &mut PgConnection) -> Result<Todo> {
    let todo = Todo::new(conn, "Produce Software")?;
    todo.add_task(conn, "Write Code")?;
    todo.add_task(conn, "Compile Source")?;
    todo.add_task(conn, "Run Tests")?;

    Ok(todo)
}
