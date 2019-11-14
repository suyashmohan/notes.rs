use serde_derive::{Deserialize, Serialize};

use crate::schema::notes;

#[derive(Insertable)]
#[derive(Queryable)]
#[table_name="notes"]
pub struct Note {
    pub title: String,
    pub body: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewNote {
    pub title: String,
    pub body: String,
}