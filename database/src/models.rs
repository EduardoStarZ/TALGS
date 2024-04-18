use crate::schema::section;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::section)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Section {
    pub id: i32,
    pub name: String
}

#[derive(Insertable)]
#[diesel(table_name = section)]
pub struct NewSection<'a> {
    pub id : &'a i32,
    pub name: &'a String
}