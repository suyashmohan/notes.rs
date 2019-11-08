use actix_web::{Error, HttpResponse};
use futures::{future::ok as ok, Future};

pub fn get() -> impl Future<Item = HttpResponse, Error = Error> {
    ok(HttpResponse::Ok().body("Hello! World"))
}