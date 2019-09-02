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
    info!("Request comics list");
    db.send(ComicsList)
        .from_err()
        .and_then(move |res| match res {
            Ok(comics_list_msg) => Ok(HttpResponse::Ok().json(comics_list_msg.comics_list)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
}

pub fn comics_show(
    _req: HttpRequest,
    db: web::Data<Addr<DbExecutor>>,
    id: web::Path<i32>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let comics_id = ComicsId { comics_id: *id };
    info!("Request comics");
    db.send(comics_id)
        .from_err()
        .and_then(move |res| match res {
            Ok(comics_id_msg) => Ok(HttpResponse::Ok().json(comics_id_msg.comics_id)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
}
