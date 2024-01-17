use crate::db::model::Todo;
use color_eyre::Result;
use diesel::PgConnection;

mod db;

fn main() -> Result<()> {
    let mut conn = db::get_connection()?;

    // Print all todos and their tasks.
    seed_todos(&mut conn)?
        .iter()
        .try_for_each::<_, Result<()>>(|todo| {
            let tasks = todo.get_tasks(&mut conn)?;

            println!("{todo}:");
            for (i, task) in tasks.iter().enumerate() {
                println!("{i}. {task}", i = i + 1);
            }

            println!();

            Ok(())
        })?;

    Ok(())
}

fn seed_todos(conn: &mut PgConnection) -> Result<Vec<Todo>> {
    let mut todos = Vec::new();

    let todo = Todo::new(conn, "Produce Software")?;
    todo.add_task(conn, "Write Code")?;
    todo.add_task(conn, "Compile Source")?;
    todo.add_task(conn, "Run Tests")?;

    todos.push(todo);

    let todo = Todo::new(conn, "Brew Coffee")?;
    todo.add_task(conn, "Pour Water")?;
    todo.add_task(conn, "Pour Coffee")?;
    todo.add_task(conn, "Turn On")?;

    todos.push(todo);

    Ok(todos)
}
