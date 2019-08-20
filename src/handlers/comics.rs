use crate::db_connection::ConnDsl;
use crate::models::comics::Characters;
use crate::models::comics::CharactersList;
use crate::models::comics::Comics;
use crate::models::comics::ComicsList;
use actix::prelude::*;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use futures::Future;

pub fn superheros(_req: HttpRequest) -> HttpResponse {
    #[derive(Serialize)]
    struct Api {
        name: String,
        version: String,
    }

    info!("Request api info");

    let data = Api {
        name: "superheros".to_string(),
        version: "1.0".to_string(),
    };
    HttpResponse::Ok().json(data)
}

// pub fn comics_index(_req: HttpRequest) -> HttpResponse {
//     info!("Request comics list");
//     HttpResponse::Ok().json(ComicsList::list())
// }

pub fn comics_show(id: web::Path<i32>) -> Result<HttpResponse, HttpResponse> {
    info!("Request comics id: {}", &id);
    Comics::find(&id)
        .map(|product| HttpResponse::Ok().json(product))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

pub fn characters_index(_req: HttpRequest) -> HttpResponse {
    info!("Request characters list");
    HttpResponse::Ok().json(CharactersList::list())
}

pub fn characters_stats(_req: HttpRequest) -> Result<HttpResponse, HttpResponse> {
    info!("Request characters stats");
    Characters::find()
        .map(|data| HttpResponse::Ok().json(data))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

pub fn add(
    _req: HttpRequest,
    db: web::Data<Addr<ConnDsl>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    db.send(ComicsList)
        .from_err()
        .and_then(move |res| match res {
            Ok(comics_list) => Ok(HttpResponse::Ok().json(comics_list.comics_list)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
}

//pub fn comics_index_pool(
//    _req: HttpRequest,
//    pool: web::Data<DbPool>,
//) -> impl Future<Item = HttpResponse, Error = Error> {
//    //info!("Request comics list pool");
//    web::block(move || ComicsList::listpool(pool)).then(|res| match res {
//        Ok(data) => Ok(HttpResponse::Ok().json(data)),
//        Err(_) => Ok(HttpResponse::InternalServerError().into()),
//    })
//}
//
// Async request handler
//

// pub fn add(
//     name: web::Path<String>,
//     pool: web::Data<ConnDsl>,
// ) -> impl Future<Item = HttpResponse, Error = Error> {
//     // run diesel blocking code
//     web::block(move || query(pool)).then(|res| match res {
//         Ok(user) => Ok(HttpResponse::Ok().json(user)),
//         Err(_) => Ok(HttpResponse::InternalServerError().into()),
//     })
// }

// pub fn query(pool: web::Data<ConnDsl>) -> Result<Comics, diesel::result::Error> {
//     use crate::schema::comics;
//     use diesel::QueryDsl;
//     use diesel::RunQueryDsl;
//     let connection = &pool;

//     comics::table.find(2).first(&connection)
// }
