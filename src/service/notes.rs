use uuid::Uuid;

use crate::data;
use crate::model::notes;
use crate::DBPool;

pub fn create(pool: &DBPool, title: String, body: String) -> notes::Note{
    let new_note = notes::NewNote {
        id: Uuid::new_v4(),
        title, body,
    };

    data::notes::create(&pool, new_note.clone()).expect("Unable to create note")
}

pub fn get(pool: &DBPool, id: Uuid) -> notes::Note {
    data::notes::get(pool, id).expect("Unable to find note")
}