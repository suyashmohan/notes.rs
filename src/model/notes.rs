use uuid::Uuid;
use serde_derive::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use chrono::naive::serde::ts_seconds;

use crate::schema::notes;

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Note {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    #[serde(with = "ts_seconds")]
    pub created_at: NaiveDateTime,
    #[serde(with = "ts_seconds")]
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable)]
#[table_name="notes"]
pub struct NewNote {
    pub id: Uuid,
    pub title: String,
    pub body: String,
}