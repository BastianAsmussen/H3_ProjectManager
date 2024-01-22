use color_eyre::Result;
use crate::db::model::tasks::Todo;
use crate::db::model::teams::{Team, TeamWorker, Worker};

mod db;

fn main() -> Result<()> {
    let mut conn = db::get_connection()?;

    let todo = Todo::new(&mut conn, "Todo")?;
    let task = todo.add_task(&mut conn, "Task")?;

    let team = Team::new(&mut conn, "Team 1", todo.id)?;
    let worker = Worker::new(&mut conn, "Worker 1", task.id)?;

    let team_worker = TeamWorker::new(&mut conn, team.id, worker.id)?;

    println!("{team_worker:#?}");

    Ok(())
}
