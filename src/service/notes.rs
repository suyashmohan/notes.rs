use crate::data;
use crate::model;

pub fn create(title: &str, text: &str) -> model::notes::Note {
    data::notes::create(title.to_string(), text.to_string())
}