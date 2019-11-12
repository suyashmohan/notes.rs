use crate::schema::notes;

#[derive(Insertable)]
#[derive(Queryable)]
#[table_name="notes"]
pub struct Note<'a> {
    pub title: &'a str,
    pub body: &'a str,
}