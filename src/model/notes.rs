use uuid::Uuid;
use chrono::NaiveDateTime;

use crate::schema::notes;

#[derive(Debug, Queryable)]
pub struct Note {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name="notes"]
pub struct NewNote {
    pub id: Uuid,
    pub title: String,
    pub body: String,
}