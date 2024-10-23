use core::{panic, str};
use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::Write,
    path::Path,
};

use crate::td::{self, record_to_todo, Todo};

// Should only be used by the create_file_if_not_exists function,
// it checks if the td.csv file exists, and if it doesnt,
// returns a file to be used
fn get_path_create(file: &str) -> Result<File, &'static str> {
    let path = Path::new(file);
    if path.exists() {
        return Err("file already exists");
    }
    let file = OpenOptions::new().append(true).create(true).open(file);
    match file {
        Ok(f) => Ok(f),
        Err(e) => panic!("{}", e),
    }
}

// Helper made to reduce amount of code, this gets called by every function
// that interacts with todos
fn get_path(file: &str) -> File {
    let file = OpenOptions::new().append(true).read(true).open(file);
    match file {
        Ok(res) => res,
        Err(e) => panic!("{}", e),
    }
}

fn get_path_overwrite(file: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file);
    match &mut file {
        Ok(f) => f
            .write_all(b"id, title, content, situation, creation_date, completion_date\n")
            .unwrap(),
        Err(e) => panic!("{}", e),
    }
}

// Defaults
pub fn create_file_if_not_exists(file: &str) {
    let mut f = get_path_create(file);
    match &mut f {
        Ok(f) => f
            .write_all(b"id, title, content, situation, creation_date, completion_date\n")
            .unwrap(),
        Err(_e) => {}
    }
}

// Reads the contents of td.csv and
// sends them to be turned into a Todo struct
pub fn read_file_and_return_todos(file_path: &str) -> Result<Vec<td::Todo>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(file_path)?;
    let mut return_values: Vec<td::Todo> = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let rec = record.deserialize::<Vec<String>>(None).unwrap();
        return_values.push(td::record_to_todo(rec));
    }
    Ok(return_values)
}

// Receives a filepath and a bytes vector from main, transforms it into a csv
// and writes it to the file on filepath
pub fn save_todo_to_file(file: &str, value_list: Vec<u8>) -> Result<(), Box<dyn Error>> {
    let f = get_path(file);

    let mut wrtr = csv::Writer::from_writer(f);
    let decoded: Todo = bincode::deserialize(&value_list).unwrap();

    let value_list = [
        get_next_id(file),
        decoded.title,
        decoded.content,
        decoded.situation.to_string(),
        decoded.creation_date.get_formatted(),
        decoded.completion_date.get_formatted(),
    ];

    wrtr.write_record(value_list)?;
    wrtr.flush()?;
    Ok(())
}

fn get_next_id(file: &str) -> String {
    read_file_and_return_todos(file).unwrap().len().to_string()
}

pub fn clear_todo_file(file: &str) -> Result<(), Box<dyn Error>> {
    get_path_overwrite(file);
    Ok(())
}
