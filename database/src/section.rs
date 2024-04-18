use crate::models::{NewSection, Section};
use crate::schema::section::{self, dsl::*};
use diesel::prelude::*;

pub fn sync_local_sections(
    local_connection: &mut SqliteConnection,
    remote_connection: &mut SqliteConnection,
) {
    let local_sections: Vec<Section> = section
        .select(Section::as_select())
        .load(local_connection)
        .expect("could not load table section");

    let values: (Vec<i32>, Vec<String>) = {
        let mut ids: Vec<i32> = Vec::new();
        let mut names: Vec<String> = Vec::new();

        for x in local_sections {
            ids.push(x.id);
            names.push(x.name);
        }

        (ids, names)
    };

    let remote_unique_sections: Vec<Section> = section
        .select(Section::as_select())
        .filter(id.ne_all(values.0).and(name.ne_all(values.1)))
        .load(remote_connection)
        .expect("could not load remote table section");

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

    
    let new_section : NewSection<'_> = NewSection {
        id: &(results.len() as i32),
        name: &namespace.trim().to_string(),
    };

    diesel::insert_into(section::table)
        .values(&new_section)
        .execute(connection)
        .unwrap();
}