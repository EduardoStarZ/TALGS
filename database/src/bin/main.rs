use database::section::*;
use database::*;
use diesel::SqliteConnection;

fn main() {
    let start : Vec<u128> = counter();

    let local_conn : &mut SqliteConnection = &mut establish_local_connection();
    let remote_conn : &mut SqliteConnection = &mut establish_remote_connection();

    //create_section(remote_conn, &"Bebidas".to_string());

    //sync_local_sections(local_conn, remote_conn);

    println!("Local DB: ");
    list_sections(local_conn);

    println!("Remote DB: ");
    list_sections(remote_conn);

    //kill_all(true, local_conn, remote_conn);

    let end : Vec<u128> = counter();

    println!("Database-sync was executed in {} ms ({} ns, {} mcs)", (end[2] - start[2]) ,(end[0] - start[0]), (end[1] -  start[1]));
}

fn kill_all(killall : bool, local_conn : &mut SqliteConnection, remote_conn : &mut SqliteConnection) {
   if killall {
        drop_sections(local_conn);
        drop_sections(remote_conn);
   } 
}
