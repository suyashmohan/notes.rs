use actix_web::{web, Error, HttpResponse};
use futures::{future::ok as ok, Future};

use crate::service::notes;
use crate::model;
use crate::DBPool;

pub fn create(note_data: web::Json<model::notes::NewNote>, pool: web::Data<DBPool>) -> impl Future<Item = HttpResponse, Error = Error> {
    let note = notes::create(&pool, note_data.title.clone(), note_data.body.clone());
    ok(HttpResponse::Ok().body(note.body))
}