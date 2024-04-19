use crate::models::{NewSection, Section};
use crate::schema::section::{self, dsl::*};
use diesel::prelude::*;

pub fn sync_local_sections(
    local_connection: &mut SqliteConnection,
    remote_connection: &mut SqliteConnection,
) {
    //Vector with all instances of sections on the local database
    let local_sections: Vec<Section> = section
        .select(Section::as_select())
        .load(local_connection)
        .expect("could not load table section");

    //Tuple with the values of local instances of sections
    let values: (Vec<i32>, Vec<String>) = {
        let mut ids: Vec<i32> = Vec::new();
        let mut names: Vec<String> = Vec::new();

        for x in local_sections.clone() {
            ids.push(x.id);
            names.push(x.name);
        }

        (ids, names)
    };

    //Memory release of previous vector
    drop(local_sections);

    /*
     * Vector with all instances of sections on the remote database
     * function ne() essentially does a NOT IN sql clause, allowing
     * us to check when there isn't a instance of the row on the remote database
     */
    let remote_unique_sections: Vec<Section> = section
        .select(Section::as_select())
        .filter(id.ne_all(values.0).and(name.ne_all(values.1)))
        .load(remote_connection)
        .expect("could not load remote table section");

    /*
     *  Looping through each instance of a section row that isn't present in the local database
     * and inserting them into the local database section table
     */
    for x in remote_unique_sections {
        diesel::insert_into(section::table)
            .values(NewSection {
                id: &x.id,
                name: &x.name,
            })
            .execute(local_connection)
            .expect("could not apply values");
    }
}

pub fn create_section(connection: &mut SqliteConnection, namespace: &String) {
    let results: Vec<Section> = section
        .select(Section::as_select())
        .load(connection)
        .expect("could not load profiles from database");

    let new_section: NewSection<'_> = NewSection {
        id: &(results.len() as i32),
        name: &namespace.trim().to_string(),
    };

    diesel::insert_into(section::table)
        .values(&new_section)
        .execute(connection)
        .unwrap();
}

pub fn delete_section(connection : &mut SqliteConnection, id_num : i32) {
    let result : Vec<Section> = section
        .select(Section::as_select())
        .filter(section::id.eq(id_num))
        .load(connection)
        .expect("could not load table section");

    if result.len() >= 1 {
        diesel::delete(section::table)
            .filter(section::id.eq(id_num))
            .execute(connection)
            .expect("could not realise delete operation");
    }
}

pub fn list_sections(connection : &mut SqliteConnection) {
    let sections : Vec<Section> = section
        .select(Section::as_select())
        .load(connection)
        .expect("could not load table section from such database");

    for x in sections {
        println!("ID: {} | \tName: {}", x.id, x.name);
    }

    println!("");
}

pub fn drop_sections(connection : &mut SqliteConnection) {
    diesel::delete(section::table)
        .execute(connection)
        .expect("could not run drop table command on table section");
}
