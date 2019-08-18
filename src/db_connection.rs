extern crate num_cpus;
extern crate r2d2;
extern crate r2d2_postgres;
use actix::prelude::*;
use actix::SyncArbiter;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use r2d2::Pool;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok(); // This will load our .env file.

    // Load the DATABASE_URL env variable into database_url, in case of error
    // it will through a message "DATABASE_URL must be set"
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

// This is db executor actor. We are going to run 3 of them in parallel.
pub struct DbExecutor(pub Pool<r2d2_postgres::PostgresConnectionManager>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

pub fn start_database() -> Addr<DbExecutor> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager =
        r2d2_postgres::PostgresConnectionManager::new(database_url, r2d2_postgres::TlsMode::None)
            .unwrap();

    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    SyncArbiter::start(num_cpus::get() * 3, move || DbExecutor(pool.clone()))
}
