use database::section::*;
use database::*;
use diesel::SqliteConnection;

fn main() {
    let start : u128 = counter();


    let local_conn : &mut SqliteConnection = &mut establish_local_connection();
    let remote_conn : &mut SqliteConnection = &mut establish_remote_connection();

    sync_local_sections(local_conn, remote_conn);

    let end : u128 = counter();

    println!("Exec : {}", end - start)
}
