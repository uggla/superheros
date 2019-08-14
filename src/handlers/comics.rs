use actix_web::web;
use actix_web::{HttpRequest, HttpResponse};

use crate::models::comics::ComicsList;

pub fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json(ComicsList::list())
}

use crate::models::comics::Comics;

pub fn show(id: web::Path<i32>) -> Result<HttpResponse, HttpResponse> {
    Comics::find(&id)
        .map(|product| HttpResponse::Ok().json(product))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}
