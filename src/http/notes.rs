use actix_web::{web, Error, HttpResponse};
use futures::{future::ok as ok, Future};
use serde_derive::{Deserialize, Serialize};

use crate::service::notes;
use crate::DBPool;

#[derive(Serialize, Deserialize)]
pub struct CreateNoteParams {
    pub title: String,
    pub body: String,
}

pub fn create(note_data: web::Json<CreateNoteParams>, pool: web::Data<DBPool>) -> impl Future<Item = HttpResponse, Error = Error> {
    let note = notes::create(&pool, note_data.title.clone(), note_data.body.clone());
    ok(HttpResponse::Ok().body(note.body))
}