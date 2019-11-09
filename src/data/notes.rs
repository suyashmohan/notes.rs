use crate::model::notes;

pub fn create(title: String, text: String) -> notes::Note {
    notes::Note {
        title, text
    }
}