use crate::data;
use crate::model;
use crate::Pool;

pub fn create<'a>(pool: &Pool, title: &'a str, text: &'a str) -> model::notes::Note<'a> {
    data::notes::create(&pool, title, text)
}