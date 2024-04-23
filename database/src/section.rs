use crate::models::{NewSection, Section};
use crate::schema::section::{self, dsl::*};
use diesel::prelude::*;

pub async fn sync_sections(
    connection_to_be_synced: &mut SqliteConnection,
    reference_connection: &mut SqliteConnection,
) {
    //Vector with all instances of sections on the local database
    let local_sections: Vec<Section> = section
        .select(Section::as_select())
        .load(connection_to_be_synced)
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
        .load(reference_connection)
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
            .execute(connection_to_be_synced)
            .expect("could not apply values");
    }
}

/*
 * A function that returns the section table as an vector of tuples, where value 0 is the id
 * and value 1 is the name
 */
async fn section_as_tuple_vec(connection: &mut SqliteConnection) -> Vec<(i32, String)> {
    let local_sections: Vec<Section> = section
        .select(Section::as_select())
        .load(connection)
        .expect("could not load table section");

    let mut results: Vec<(i32, String)> = Vec::new();

    for x in local_sections {
        results.push((x.id, x.name));
    }

    return results;
}

/*
 * Function that creates a section row if the provided name does not exist in the database already
 */
pub async fn create_section(connection: &mut SqliteConnection, namespace: &String) {
    let results: Vec<(i32, String)> = section_as_tuple_vec(connection).await;

    if results
        .iter()
        .map(|x| x.1.clone())
        .collect::<Vec<String>>()
        .contains(namespace)
    {
        println!("\n Value \"{namespace}\" already exists at this table, skipping...");
        return;
    }

    let new_section: NewSection<'_> = NewSection {
        id: &(results.len() as i32),
        name: &namespace.trim().to_string(),
    };

    diesel::insert_into(section::table)
        .values(&new_section)
        .execute(connection)
        .unwrap();
}

/*
 * A section that deletes a row from the database based on it's id
 */
pub async fn delete_section(connection: &mut SqliteConnection, id_num: i32) {
    let result: Vec<Section> = section
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

/*
 * A function that updates a section name based on it's id
 */ 
pub async fn update_section(connection : &mut SqliteConnection, row_id : i32, new_name : String) {
    diesel::update(section)
    .filter(id.eq(row_id))
    .set(name.eq(new_name))
    .execute(connection)
    .expect("could not access table section");
} 

/*
 *  A function that displays the entire table contents
 */
pub async fn list_sections(connection: &mut SqliteConnection) {
    let sections: Vec<Section> = section
        .select(Section::as_select())
        .load(connection)
        .expect("could not load table section from such database");

    for x in sections {
        println!("ID: {} | \tName: {}", x.id, x.name);
    }

    println!("");
}

/*
 * A function that drops the table
 */
pub async fn drop_sections(connection: &mut SqliteConnection) {
    diesel::delete(section::table)
        .execute(connection)
        .expect("could not run drop table command on table section");
}
