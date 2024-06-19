use diesel::SqliteConnection;


/// A Enum defined to give a better error reporting on the implementations of the CRUD struct
pub enum ResultCode {
    ConnectionError,
    ValueError,
    Exists,
}

///A trait implemented by every table struct to allow for simple CRUD interactions with the other
///parts of the code
pub trait Crud {
    

    ///A function that creates a new item on the table if there isn't one with the given id and 
    ///returns a ResultCode or None
    fn create(&self , connection : &mut SqliteConnection) -> Option<ResultCode>;
    

    ///A function that checks if a item exists in the table and returns a ResultCode or None
    fn exists(&self, connection : &mut SqliteConnection) -> Option<ResultCode>;

    
    ///A function that updates a value in the table if it exists and returns a ResultCode or None
    fn update(&self , connection : &mut SqliteConnection) -> Option<ResultCode>;


    ///A function that deletes a value in the table if it exists and returns a ResultCode or None
    fn delete(&self, connection : &mut SqliteConnection) -> Option<ResultCode>;
}
