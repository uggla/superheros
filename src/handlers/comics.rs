//use crate::models::comics::Characters;
use crate::models::comics::Characters;
use crate::models::comics::CharactersList;
use crate::models::comics::Comics;
use crate::models::comics::ComicsList;
use actix_web::web;
use actix_web::{HttpRequest, HttpResponse};

pub fn comics_index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json(ComicsList::list())
}

pub fn comics_show(id: web::Path<i32>) -> Result<HttpResponse, HttpResponse> {
    Comics::find(&id)
        .map(|product| HttpResponse::Ok().json(product))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

pub fn characters_index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json(CharactersList::list())
}

pub fn characters_stats(_req: HttpRequest) -> Result<HttpResponse, HttpResponse> {
    Characters::find()
        .map(|data| HttpResponse::Ok().json(data))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}
