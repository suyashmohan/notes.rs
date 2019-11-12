use diesel::prelude::*;
use crate::model::notes;
use crate::Pool;
use crate::schema;

pub fn create<'a>(pool: &Pool, title: &'a str, body: &'a str) -> notes::Note<'a> {
    let new_note = notes::Note {
        title, body
    };

    let conn: &PgConnection = &pool.get().unwrap();

    diesel::insert_into(schema::notes::table)
        .values(&new_note)
        .execute(conn)
        .expect("Error saving new post");

    new_note
}