use actix_web::{HttpRequest, HttpResponse};

use crate::models::comics::ComicsList;

pub fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json(ComicsList::list())
}
