#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate actix;
extern crate actix_web;
extern crate futures;
#[macro_use]
extern crate log;

pub mod db_connection;
pub mod handlers;
mod logger;
pub mod models;
pub mod schema;

use actix_web::{web, App, HttpServer};
use log::LevelFilter;
use logger::init_log;

fn main() {
    init_log(LevelFilter::Debug).expect("Error initializing logger");

    let sys = actix::System::new("superheros");
    let db_addr = db_connection::get_db_addr();

    info!("Superheros API !!!");
    HttpServer::new(move || {
        App::new()
            .data(db_addr.clone())
            .service(web::resource("/").route(web::get().to_async(handlers::comics::superheros)))
            .service(
                web::resource("/Comics/{id}")
                    .route(web::get().to_async(handlers::comics::comics_show)),
            )
            .service(
                web::resource("/Characters/list")
                    .route(web::get().to_async(handlers::comics::characters_index)),
            )
            .service(
                web::resource("/Characters")
                    .route(web::get().to_async(handlers::comics::characters_stats)),
            )
            .service(
                web::resource("/Comics").route(web::get().to_async(handlers::comics::comics_list)),
            )
    })
    .bind("0.0.0.0:8088")
    .expect("Could not start web server !")
    .start();

    let _ = sys.run();
}
