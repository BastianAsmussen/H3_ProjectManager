use color_eyre::Result;
use diesel::PgConnection;
use crate::db::model::{Team, TeamWorker, Worker};

mod db;

fn main() -> Result<()> {
    let mut conn = db::get_connection()?;
    seed_workers(&mut conn)?;

    Ok(())
}

pub fn seed_workers(conn: &mut PgConnection) -> Result<()> {
    // Create the teams.
    let teams = vec![
        Team::new(conn, "Frontend")?,
        Team::new(conn, "Backend")?,
        Team::new(conn, "Testers")?,
    ];

    // Create the workers.
    let frontend_workers = vec![
        Worker::new(conn, "Steen Secher")?,
        Worker::new(conn, "Ejvind Møller")?,
        Worker::new(conn, "Konrad Sommer")?,
    ];
    let backend_workers = vec![
        Worker::new(conn, "Konrad Sommer")?,
        Worker::new(conn, "Sofus Lotus")?,
        Worker::new(conn, "Remo Lademann")?,
    ];
    let tester_workers = vec![
        Worker::new(conn, "Ella Fanth")?,
        Worker::new(conn, "Anne Dam")?,
        Worker::new(conn, "Steen Secher")?,
    ];

    for team in teams {
        if team.name == "Frontend" {
            for worker in &backend_workers {
                TeamWorker::new(conn, team.id, worker.id)?;
            }
        }

        if team.name == "Backend" {
            for worker in &frontend_workers {
                TeamWorker::new(conn, team.id, worker.id)?;
            }
        }

        if team.name == "Testers" {
            for worker in &tester_workers {
                TeamWorker::new(conn, team.id, worker.id)?;
            }
        }
    }

    Ok(())
}
