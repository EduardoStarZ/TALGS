use diesel::SqliteConnection;

pub trait Crud {
    fn create(&self , connection : &mut SqliteConnection);
    fn update(&self , connection : &mut SqliteConnection);
    fn delete(&self, connection : &mut SqliteConnection);
}
