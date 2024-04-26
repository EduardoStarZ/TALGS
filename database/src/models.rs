use crate::schema::section;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::section)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Clone)]
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


pub struct Product {
    pub id: i32,
    pub name: String,
    pub total_amount : i32,
    pub measure_unit : String,
    pub measure : f32,
    pub section_id : i32
}
