use diesel::SqliteConnection;

pub enum ResultCode {
    ConnectionError,
    ValueError,
    Exists,
}

pub trait Crud {
    fn create(&self , connection : &mut SqliteConnection) -> Option<ResultCode>;
    fn exists(&self, connection : &mut SqliteConnection) -> Option<ResultCode>;
    fn update(&self , connection : &mut SqliteConnection) -> Option<ResultCode>;
    fn delete(&self, connection : &mut SqliteConnection) -> Option<ResultCode>;
}
