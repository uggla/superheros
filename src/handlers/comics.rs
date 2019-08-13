use actix_web::{HttpRequest, HttpResponse};

use crate::models::comics::ComicList;

pub fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json(ComicList::list())
}
