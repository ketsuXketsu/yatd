mod td;
mod tdcsv;
mod tdterminal;

use core::panic;

use td::Situation;
use tdterminal::Terminal;

fn main() {
    let filepath = "./td.csv";
    tdcsv::create_file_if_not_exists(filepath);

    loop {
        println!(
            "commands : \n
                w -> write \n 
                e -> edit \n 
                d -> delete \n
                g -> get by id \n 
                G -> get all \n
                a -> set as finished \n 
                A -> set as unfinished \n
                q -> quit"
        );

        let input = Terminal::get_input().unwrap();
        match input.as_ref() {
            "w" => td_write(filepath),
            "e" => td_edit(filepath),
            "d" => td_delete(filepath),
            "g" => match td_get_by_id(filepath) {
                None => println!("no todo with this id"),
                Some(todo) => todo.pretty_print(),
            },
            "G" => {
                td_get_all(filepath).map(|todos| println!("{:?}", todos));
            }
            "a" => td_change_situation(1, filepath),
            "A" => td_change_situation(0, filepath),
            "q" => return,
            "clear" | "cls" => Terminal::clear(),
            _ => {
                println!("invalid command")
            }
        }
    }
}

// Receives info to create a new todo, encodes it into bytes
// and sends it to tdcsv
fn td_write(filepath: &str) {
    let mut new_todo = td::new_todo();
    println!("enter the title");
    let mut input = Terminal::get_input().unwrap();
    new_todo.change_title(&input);

    println!("enter the content");
    input = Terminal::get_input().unwrap();
    new_todo.change_content(&input);

    let encoded: Vec<u8> = bincode::serialize(&new_todo).unwrap();
    tdcsv::save_todo_to_file(filepath, encoded).unwrap();
}

fn td_edit(filepath: &str) {
    let mut editing_todo = td_get_by_id(filepath).unwrap();
    println!("enter a new title");
    let mut input = Terminal::get_input().unwrap();
    editing_todo.change_title(&input);

    println!("enter a new content");
    input = Terminal::get_input().unwrap();
    editing_todo.change_content(&input);

    let mut value_list = get_todo_list(filepath).unwrap();
    value_list.iter_mut().for_each(|todo| {
        if todo.id == editing_todo.id {
            *todo = editing_todo.clone();
        }
    });

    tdcsv::clear_todo_file(filepath).unwrap();
    for value in value_list {
        let encoded: Vec<u8> = bincode::serialize(&value).unwrap();
        tdcsv::save_todo_to_file(filepath, encoded).unwrap();
    }
}

fn td_get_by_id(file: &str) -> Option<td::Todo> {
    println!("insert id");
    let input_id: i32 = Terminal::get_input().unwrap().parse().unwrap();
    get_todo_by_id(file, input_id).map(|todo| todo)
}

fn td_change_situation(which: i8, filepath: &str) {
    let mut editing_todo = td_get_by_id(filepath).unwrap();
    match which {
        0 => editing_todo.situation = Situation::Unfinished,
        1 => editing_todo.situation = Situation::Finished,
        _ => panic!("invalid situation selected"),
    }

    let mut value_list = get_todo_list(filepath).unwrap();
    value_list.iter_mut().for_each(|todo| {
        if todo.id == editing_todo.id {
            *todo = editing_todo.clone();
        }
    });

    tdcsv::clear_todo_file(filepath).unwrap();
    for value in value_list {
        let encoded: Vec<u8> = bincode::serialize(&value).unwrap();
        tdcsv::save_todo_to_file(filepath, encoded).unwrap();
    }
}

fn td_delete(file: &str) {
    let todo_list = tdcsv::read_file_and_return_todos(file).unwrap();
    println!("insert todo id");
    let id: i32 = Terminal::get_input().unwrap().parse().unwrap();
    tdcsv::clear_todo_file(file);
    for todo in todo_list {
        if todo.id != id {
            let encoded_todo = bincode::serialize(&todo).unwrap();
            tdcsv::save_todo_to_file(file, encoded_todo);
        }
    }
}

// Middle between main and tdcsv, gets all output from read_file_and_return_todos
fn get_todo_list(file: &str) -> Option<Vec<td::Todo>> {
    Some(tdcsv::read_file_and_return_todos(file).unwrap())
}

fn td_get_all(file: &str) -> Option<Vec<td::Todo>> {
    Some(get_todo_list(file).unwrap())
}

fn get_todo_by_id(file: &str, id: i32) -> Option<td::Todo> {
    match get_todo_list(file) {
        None => None,
        Some(todos) => {
            for todo in todos {
                if todo.id == id {
                    return Some(todo);
                }
            }
            None
        }
    }
}
