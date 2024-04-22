use database::section::*;
use database::*;
use diesel::SqliteConnection;
use std::env;

fn get_arg() -> String {
    env::args().collect::<Vec<String>>().get(1).expect(" ").clone()
}

#[tokio::main]
async fn main() {
    let start: Vec<u128> = counter();

    
    let mut handles = vec![];
    
    for _ in 0..1 {
        let handle = tokio::spawn(async move {
            let local_conn: &mut SqliteConnection = &mut establish_local_connection();
            let remote_conn: &mut SqliteConnection = &mut establish_remote_connection();
            
            create_section(local_conn, &get_arg()).await;

            sync_sections(remote_conn, local_conn).await;

            println!("Local DB: ");
            list_sections(local_conn).await;

            println!("Remote DB: ");
            list_sections(remote_conn).await;

            kill_all(true, local_conn, remote_conn).await;
        });
        handles.push(handle);
    }

    let end: Vec<u128> = counter();

    println!(
        "Database-sync was executed in {} ms ({} ns, {} mcs)",
        (end[2] - start[2]),
        (end[0] - start[0]),
        (end[1] - start[1])
    );
}

async fn kill_all(killall: bool, local_conn: &mut SqliteConnection, remote_conn: &mut SqliteConnection) {
    if killall {
        drop_sections(local_conn).await;
        drop_sections(remote_conn).await;
    }
}
