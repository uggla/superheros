use crate::models::comics::Comics;
use crate::models::comics::ComicsList;
use actix_web::web;
use actix_web::{HttpRequest, HttpResponse};

pub fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json(ComicsList::list())
}

pub fn show(id: web::Path<i32>) -> Result<HttpResponse, HttpResponse> {
    Comics::find(&id)
        .map(|product| HttpResponse::Ok().json(product))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}
