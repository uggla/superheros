pub mod db_connection;
pub mod handlers;
pub mod models;
pub mod schema;

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
use actix_web::{web, App, HttpServer};

fn main() {
    let sys = actix::System::new("superheros");

    HttpServer::new(|| {
        App::new()
            .service(web::resource("/Comics").route(web::get().to_async(handlers::comics::index)))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .start();

    println!("Started http server: 127.0.0.1:8088");
    let _ = sys.run();
}
