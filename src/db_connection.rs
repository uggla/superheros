extern crate diesel;
extern crate num_cpus;
use crate::diesel::Connection;
use actix::*;
use diesel::prelude::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok(); // This will load our .env file.

    // Load the DATABASE_URL env variable into database_url, in case of error
    // it will through a message "DATABASE_URL must be set"
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    diesel::pg::PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

//  r2d2_diesel
pub struct ConnDsl(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for ConnDsl {
    type Context = SyncContext<Self>;
}

pub fn get_db_addr() -> Addr<ConnDsl> {
    dotenv().ok(); // This will load our .env file.
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool = Pool::builder()
        .max_size(12)
        .build(manager)
        .expect("Failed to create pool.");
    dbg!(num_cpus::get());
    SyncArbiter::start(num_cpus::get() * 3, move || ConnDsl(pool.clone()))
}
