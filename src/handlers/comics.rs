use crate::db_connection::DbExecutor;
use crate::models::comics::Characters;
use crate::models::comics::CharactersList;
use crate::models::comics::ComicsId;
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

// pub fn comics_show(id: web::Path<i32>) -> Result<HttpResponse, HttpResponse> {
//     info!("Request comics id: {}", &id);
//     Comics::find(&id)
//         .map(|product| HttpResponse::Ok().json(product))
//         .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
// }

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

pub fn comics_list(
    _req: HttpRequest,
    db: web::Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    db.send(ComicsList)
        .from_err()
        .and_then(move |res| match res {
            Ok(comics_list) => Ok(HttpResponse::Ok().json(comics_list.comics_list)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
}

pub fn comics_show(
    _req: HttpRequest,
    db: web::Data<Addr<DbExecutor>>,
    id: web::Path<i32>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let comics = ComicsId { comics_id: *id };
    db.send(comics).from_err().and_then(move |res| match res {
        Ok(comics) => Ok(HttpResponse::Ok().json(comics.comics_id)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}
