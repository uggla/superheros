use crate::models::comics::Characters;
use crate::models::comics::CharactersList;
use crate::models::comics::Comics;
use crate::models::comics::ComicsList;
use actix_web::web;
use actix_web::{HttpRequest, HttpResponse};

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

pub fn comics_index(_req: HttpRequest) -> HttpResponse {
    info!("Request comics list");
    HttpResponse::Ok().json(ComicsList::list())
}

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
