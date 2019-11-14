use diesel::prelude::*;
use crate::model::notes;
use crate::DBPool;
use crate::schema;

pub fn create(pool: &DBPool, title: String, body: String) -> notes::Note {
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