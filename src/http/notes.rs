use actix_web::{web, Error, HttpResponse};
use futures::{future::ok as ok, Future};

use crate::service::notes;
use crate::Pool;

pub fn create(pool: web::Data<Pool>) -> impl Future<Item = HttpResponse, Error = Error> {
    let note = notes::create(&pool, "Todo", "Make it work later");
    ok(HttpResponse::Ok().body(note.body))
}