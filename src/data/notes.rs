use diesel::prelude::*;
use diesel::result::QueryResult;
use uuid::Uuid;

use crate::model::notes;
use crate::DBPool;
use crate::schema;

pub fn create(pool: &DBPool, new_note: notes::NewNote) -> QueryResult<notes::Note> {
    let conn: &PgConnection = &pool.get().unwrap();

    diesel::insert_into(schema::notes::table)
        .values(&new_note)
        .get_result::<notes::Note>(conn)
}

pub fn get(pool: &DBPool, id: Uuid) -> QueryResult<notes::Note> {
    let conn: &PgConnection = &pool.get().unwrap();

    schema::notes::dsl::notes
        .find(id)
        .get_result(conn)
}

pub fn update(pool: &DBPool, update_note: notes::NewNote) -> QueryResult<notes::Note> {
    let conn: &PgConnection = &pool.get().unwrap();

    let target = schema::notes::dsl::notes
        .find(update_note.id);

    diesel::update(target)
        .set((
            schema::notes::dsl::title.eq(update_note.title),
            schema::notes::dsl::body.eq(update_note.body)
        ))
        .get_result::<notes::Note>(conn)
}