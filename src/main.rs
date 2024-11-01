#[allow(clippy::pedantic)]
mod csv_handler;
mod terminal;
mod todo;
use core::panic;
use terminal::Terminal;
use todo::{new_todo, Situation};

fn main() {
    let tdops = TdOps {
        file: ".td.csv".to_string(),
    };
    csv_handler::create_file_if_not_exists(&tdops.file);

    loop {
        println!(
            "\n
------------------\n
commands : \n
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
            "w" => tdops.write(),
            "e" => tdops.edit(),
            "d" => tdops.delete(),
            "g" => match tdops.get_by_id() {
                None => println!("no todo with this id"),
                Some(todo) => todo.pretty_print(true),
            },
            "G" => {
                tdops.get_all().map(|todos| tdops.pretty_print_list(&todos));
            }
            "a" => tdops.change_situation(1),
            "A" => tdops.change_situation(0),
            "q" => return,
            "clear" | "cls" => Terminal::clear(),
            _ => {
                println!("invalid command")
            }
        }
    }
}

// Object for the Todo operations
struct TdOps {
    file: String,
}

impl TdOps {
    // Middle between main and tdcsv, gets all output from read_file_and_return_todos
    fn get_todo_list(&self) -> Option<Vec<todo::Todo>> {
        Some(csv_handler::read_file_and_return_todos(&self.file).unwrap())
    }

    fn get_all(&self) -> Option<Vec<todo::Todo>> {
        Some(self.get_todo_list().unwrap())
    }

    fn get_todo_by_id(&self, id: i32) -> Option<todo::Todo> {
        match self.get_todo_list() {
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

    // Receives info to create a new todo, encodes it into bytes
    // and sends it to tdcsv
    fn write(&self) {
        let mut new_todo = todo::new_todo();
        println!("enter the title");
        let mut title_input = Terminal::get_input();
        // Match necessary in case user quits input mode
        match &mut title_input {
            Err(e) => println!("{}", e),
            Ok(input) => {
                new_todo.change_title(input);
                println!("enter the content");
                let mut content_input = Terminal::get_input();

                // Match necessary in case user quits input mode
                match &mut content_input {
                    Err(e) => println!("{}", e),
                    Ok(input) => {
                        new_todo.change_content(input);
                        new_todo.creation_date = new_todo.update_date();
                        let encoded: Vec<u8> = bincode::serialize(&new_todo).unwrap();
                        csv_handler::save_todo_to_file(&self.file, encoded).unwrap();
                    }
                }
            }
        }
    }

    fn edit(&self) {
        let mut editing_todo = self.get_by_id().unwrap();
        println!("enter a new title");
        let mut input = Terminal::get_input().unwrap();
        editing_todo.change_title(&input);

        println!("enter a new content");
        input = Terminal::get_input().unwrap();
        editing_todo.change_content(&input);

        let mut value_list = self.get_todo_list().unwrap();
        value_list.iter_mut().for_each(|todo| {
            if todo.id == editing_todo.id {
                *todo = editing_todo.clone();
            }
        });

        csv_handler::clear_todo_file(&self.file).unwrap();
        for value in value_list {
            let encoded: Vec<u8> = bincode::serialize(&value).unwrap();
            csv_handler::save_todo_to_file(&self.file, encoded).unwrap();
        }
    }

    fn get_by_id(&self) -> Option<todo::Todo> {
        println!("insert id");
        let input = Terminal::get_input();
        match input {
            Err(e) => {
                println!("{}", e);
                None
            }
            Ok(input) => {
                let id: i32 = input.parse().unwrap();
                self.get_todo_by_id(id).map(|todo| todo)
            }
        }
    }

    fn change_situation(&self, which: i8) {
        let mut editing_todo = self.get_by_id().unwrap();
        match which {
            0 => editing_todo.situation = Situation::Unfinished,
            1 => editing_todo.situation = Situation::Finished,
            _ => panic!("invalid situation selected"),
        }

        let mut value_list = self.get_todo_list().unwrap();
        value_list.iter_mut().for_each(|todo| {
            if todo.id == editing_todo.id {
                *todo = editing_todo.clone();
            }
        });

        csv_handler::clear_todo_file(&self.file).unwrap();
        for value in value_list {
            let encoded: Vec<u8> = bincode::serialize(&value).unwrap();
            csv_handler::save_todo_to_file(&self.file, encoded).unwrap();
        }
    }

    fn delete(&self) {
        let todo_list = csv_handler::read_file_and_return_todos(&self.file).unwrap();
        println!("insert todo id");
        let id = self.get_by_id();
        match id {
            None => println!("no todo found with the selecte id"),
            Some(todo) => {
                csv_handler::clear_todo_file(&self.file);
                for todo in todo_list {
                    if todo.id != todo.id {
                        let encoded_todo = bincode::serialize(&todo).unwrap();
                        csv_handler::save_todo_to_file(&self.file, encoded_todo);
                    }
                }
            }
        }
    }

    fn pretty_print_list(&self, todo_list: &[todo::Todo]) {
        for todo in todo_list {
            todo.pretty_print(false);
            println!("------------");
        }
    }
}
