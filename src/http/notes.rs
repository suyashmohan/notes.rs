use actix_web::{Error, HttpResponse};
use futures::{future::ok as ok, Future};

use crate::service::notes;

pub fn create() -> impl Future<Item = HttpResponse, Error = Error> {
    let note = notes::create( "Todo", "Make it work later");
    ok(HttpResponse::Ok().body(note.text))
}