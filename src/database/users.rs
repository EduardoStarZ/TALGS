use diesel::prelude::*;
use crate::schema::auth::users;
use super::models::Crud;
use super::models::ResultCode;


///A struct defined for CRUD implementations of the users table
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



impl Crud for User {
    fn create(&self, connection : &mut SqliteConnection) -> Option<ResultCode> { 
        match self.exists(connection) {
            Some(value) => return Some(value),
            None => (),
        }
        
        match diesel::insert_into(users::table)
            .values(self)
            .execute(connection) {
                Ok(_) => None,
                Err(err) => {
                    eprintln!("Error with the database: {err}");
                    return Some(ResultCode::ConnectionError);
                }
            }
    }

    fn exists(&self, connection : &mut SqliteConnection) -> Option<ResultCode> {
        let q_users : Vec<User> = match users::table
            .filter(users::id.eq(self.id))
            .select(User::as_select())
            .load(connection) {
                Ok(value) => value,
                Err(err) => {
                    eprintln!("Error with the database: {err}");
                    return Some(ResultCode::ConnectionError);
                }
            };

        return match q_users.is_empty() {
            true => None,
            false => Some(ResultCode::Exists)
        }

    }

    fn update(&self, connection : &mut SqliteConnection) -> Option<ResultCode> {
        match self.exists(connection) {
            Some(value) => {
                match value {
                    ResultCode::Exists => (),
                    _ => return Some(value)
                }
            },
            None => return Some(ResultCode::ValueError)
        }

        match diesel::update(users::table)
            .filter(users::id.eq(self.id))
            .set(self)
            .execute(connection) {
                Ok(_) => None,
                Err(err) => {
                    eprintln!("Error with the database: {err}");
                    Some(ResultCode::ConnectionError)
                }
            }
    }


fn delete(&self, connection : &mut SqliteConnection) -> Option<ResultCode> {
    match self.exists(connection) {
        Some(value) => {
            match value {
                ResultCode::Exists => (),
                _ => return Some(value)
            }
        }
        None => return Some(ResultCode::ValueError)
    }

    match diesel::delete((users::table).filter(users::id.eq(self.id)))
        .execute(connection) {
            Ok(_) => None,
            Err(err) => {
                eprintln!("Error with the database : {err}");
                Some(ResultCode::ConnectionError)
            } 
    }
}

}
