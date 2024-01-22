use diesel::{Connection, ConnectionResult, PgConnection};

mod schema;
pub mod model;

#[allow(clippy::expect_used)]
pub fn get_connection() -> ConnectionResult<PgConnection> {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set!");

    PgConnection::establish(&database_url)
}
