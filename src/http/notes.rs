use actix_web::{web, Error, HttpResponse};
use futures::{future::ok as ok, Future};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

use crate::service::notes;
use crate::DBPool;

#[derive(Serialize, Deserialize)]
pub struct CreateNoteParams {
    pub title: String,
    pub body: String,
}

#[derive(Serialize, Deserialize)]
pub struct NoteID {
    pub id: Uuid
}

#[derive(Serialize, Deserialize)]
pub struct ErrorBody {
    pub error: String,
}

pub fn create(note_data: web::Json<CreateNoteParams>, pool: web::Data<DBPool>) -> impl Future<Item = HttpResponse, Error = Error> {
    let note = notes::create(&pool, note_data.title.clone(), note_data.body.clone());
    ok(HttpResponse::Ok().json(note))
}

pub fn get(note_id: web::Path<NoteID>, pool: web::Data<DBPool>) -> impl Future<Item = HttpResponse, Error = Error> {
    match notes::get(&pool, note_id.id){
        Some(note) => ok(HttpResponse::Ok().json(note)),
        None => ok(HttpResponse::NotFound().json(ErrorBody { error: "Unable to find given ID".to_string()}))
    }
}