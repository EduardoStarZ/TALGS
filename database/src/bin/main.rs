use database::section::*;
use database::*;
use diesel::SqliteConnection;

fn main() {
    let start : Vec<u128> = counter();

    let local_conn : &mut SqliteConnection = &mut establish_local_connection();
    let remote_conn : &mut SqliteConnection = &mut establish_remote_connection();

    create_section(remote_conn, &"I am all of me".to_string());

    sync_local_sections(local_conn, remote_conn);

    let end : Vec<u128> = counter();

    println!("{} Nanoseconds\n{} Microseconds\n{} Miliseconds", (end[0] - start[0]) ,(end[1] - start[1]), (end[2] -  start[2]));
}