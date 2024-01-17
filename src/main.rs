use crate::db::model::Todo;
use color_eyre::Result;

mod db;

fn main() -> Result<()> {
    let mut conn = db::get_connection()?;

    let todo = Todo::new(&mut conn, "Shopping")?;
    let mut task = todo.add_task(&mut conn, "Get Apples")?;
    task.complete(&mut conn)?;

    println!("Todo: {todo:?}");
    println!("Task: {task:?}");

    Ok(())
}
