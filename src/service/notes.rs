use crate::data;
use crate::model;
use crate::DBPool;

pub fn create(pool: &DBPool, title: String, text: String) -> model::notes::Note{
    data::notes::create(&pool, title, text)
}