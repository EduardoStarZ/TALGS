use diesel::prelude::*;
use crate::schema::auth::users::{self, dsl::*};
use crate::database::models::Crud;

#[derive(Insertable, Selectable, Queryable, AsChangeset, Debug)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id : i32,
    pub name : String,
    pub password : String,
    pub group : i32,
    pub cpf : String
}

pub fn get_all_users(connection : &mut SqliteConnection) -> Vec<User> {
    users.select(User::as_select()).load(connection).unwrap()
}


impl<'a> Crud for User {
    fn create(&self, connection : &mut SqliteConnection) { 
        match diesel::insert_into(users::table)
            .values(self)
            .execute(connection) {
                Ok(_) => (),
                Err(err) => {
                    eprintln!("Error with the database: {err}");
                }
            }
    }

    fn update(&self, connection : &mut SqliteConnection) {
        match diesel::update(users::table)
            .filter(users::id.eq(self.id))
            .set(self)
            .execute(connection) {
                Ok(_) => (),
                Err(err) => {
                    eprintln!("Error with the database: {err}");
                }
            }
    }


    fn delete(&self, connection : &mut SqliteConnection) {
        match diesel::delete((users::table).filter(users::id.eq(self.id)))
            .execute(connection) {
                Ok(_) => (),
                Err(err) => {
                    eprintln!("Error with the database : {err}");
                } 
            }
    }
}
