use uuid::Uuid;
use diesel::prelude::*;
use diesel::result::QueryResult;
use crate::model::notes;
use crate::DBPool;
use crate::schema;

pub fn create(pool: &DBPool, title: String, body: String) -> QueryResult<notes::Note> {
    let new_note = notes::NewNote {
        id: Uuid::new_v4(),
        title, body,
    };
    println!("{:?}", new_note);
    let conn: &PgConnection = &pool.get().unwrap();

    diesel::insert_into(schema::notes::table)
        .values(&new_note)
        .get_result::<notes::Note>(conn)
}